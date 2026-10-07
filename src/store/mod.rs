//! SQLite persistence via sqlx. Every statement goes through sqlx's
//! compile-time checked macros (`query_as!` for rows, `query!` otherwise), so
//! SQL is validated against the schema in `migrations/` when dojo builds.
//!
//! Development: `scripts/dev-db.sh` builds the database the macros check
//! against; `scripts/dev-db.sh --prepare` refreshes the `.sqlx/` offline
//! cache that CI and release builds use.
//!
//! sqlx is async; the app is not. A single-threaded tokio runtime owned by
//! the store bridges the two, so callers see a plain blocking API.

use std::path::Path;
use std::time::Duration;

use anyhow::{Context, Result};
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use tokio::runtime::Runtime;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

pub struct Store {
    pool: SqlitePool,
    rt: Runtime,
}

/// Attempt counters persisted after every change, so a crash loses nothing.
pub struct AttemptRow<'a> {
    pub id: i64,
    pub active_secs: u64,
    pub test_runs: u32,
    pub failed_runs: u32,
    pub hints_used: usize,
    pub solution_viewed: bool,
    pub outcome: Option<&'a str>,
    pub code: Option<&'a str>,
}

/// What the user has done on one question before.
#[derive(Debug, Default)]
pub struct QuestionStats {
    pub attempts: i64,
    pub solved: i64,
    pub best_secs: Option<i64>,
    pub last_at: Option<String>,
}

/// An attempt that was started but never finished; it can be continued.
#[derive(Debug, Clone)]
pub struct OpenAttempt {
    pub id: i64,
    pub question_id: i64,
    pub language: String,
    pub started_at: String,
    pub active_secs: i64,
    pub test_runs: i64,
    pub failed_runs: i64,
    pub hints_used: i64,
    pub solution_viewed: bool,
    pub code: Option<String>,
}

struct Id {
    id: i64,
}

struct HistoryLine {
    line: String,
}

impl Store {
    pub fn open(path: &Path) -> Result<Store> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
        }
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .foreign_keys(true)
            .busy_timeout(Duration::from_secs(5));
        Store::connect(options).with_context(|| format!("opening {}", path.display()))
    }

    #[cfg(test)]
    pub fn memory() -> Result<Store> {
        use std::str::FromStr;
        Store::connect(SqliteConnectOptions::from_str("sqlite::memory:")?.foreign_keys(true))
    }

    fn connect(options: SqliteConnectOptions) -> Result<Store> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        // One connection: dojo is single-user, and an in-memory database
        // only exists on the connection that created it.
        let pool = rt.block_on(
            SqlitePoolOptions::new()
                .max_connections(1)
                .min_connections(1)
                .idle_timeout(None)
                .max_lifetime(None)
                .connect_with(options),
        )?;
        rt.block_on(MIGRATOR.run(&pool))
            .context("migrating the database (if it was created by a newer dojo, upgrade dojo)")?;
        Ok(Store { pool, rt })
    }

    pub fn push_history(&self, line: &str) -> Result<()> {
        let at = now();
        self.rt.block_on(
            sqlx::query!(
                "INSERT INTO input_history (at, line) VALUES (?, ?)",
                at,
                line
            )
            .execute(&self.pool),
        )?;
        Ok(())
    }

    /// Most recent distinct input lines, oldest first.
    pub fn history(&self, limit: usize) -> Result<Vec<String>> {
        let limit = limit as i64;
        let rows = self.rt.block_on(
            sqlx::query_as!(
                HistoryLine,
                r#"SELECT line AS "line!" FROM input_history GROUP BY line ORDER BY MAX(id) DESC LIMIT ?"#,
                limit
            )
            .fetch_all(&self.pool),
        )?;
        Ok(rows.into_iter().rev().map(|r| r.line).collect())
    }

    pub fn create_session(&self, mode: &str, query: Option<&str>) -> Result<i64> {
        let at = now();
        let row = self.rt.block_on(
            sqlx::query_as!(
                Id,
                r#"INSERT INTO sessions (started_at, mode, query) VALUES (?, ?, ?)
                   RETURNING id AS "id!""#,
                at,
                mode,
                query
            )
            .fetch_one(&self.pool),
        )?;
        Ok(row.id)
    }

    pub fn end_session(&self, id: i64) -> Result<()> {
        let at = now();
        self.rt.block_on(
            sqlx::query!(
                "UPDATE sessions SET ended_at = ? WHERE id = ? AND ended_at IS NULL",
                at,
                id
            )
            .execute(&self.pool),
        )?;
        Ok(())
    }

    pub fn create_attempt(&self, session_id: i64, question_id: u32, language: &str) -> Result<i64> {
        let at = now();
        let question_id = question_id as i64;
        let dojo_version = env!("CARGO_PKG_VERSION");
        let row = self.rt.block_on(
            sqlx::query_as!(
                Id,
                r#"INSERT INTO attempts (session_id, question_id, dojo_version, language, started_at)
                   VALUES (?, ?, ?, ?, ?)
                   RETURNING id AS "id!""#,
                session_id,
                question_id,
                dojo_version,
                language,
                at
            )
            .fetch_one(&self.pool),
        )?;
        Ok(row.id)
    }

    pub fn save_attempt(&self, a: &AttemptRow) -> Result<()> {
        let ended_at = a.outcome.map(|_| now());
        let active_secs = a.active_secs as i64;
        let (test_runs, failed_runs) = (a.test_runs as i64, a.failed_runs as i64);
        let hints_used = a.hints_used as i64;
        self.rt.block_on(
            sqlx::query!(
                "UPDATE attempts
                 SET active_secs = ?, test_runs = ?, failed_runs = ?, hints_used = ?,
                     solution_viewed = ?, outcome = ?, code = COALESCE(?, code), ended_at = ?
                 WHERE id = ?",
                active_secs,
                test_runs,
                failed_runs,
                hints_used,
                a.solution_viewed,
                a.outcome,
                a.code,
                ended_at,
                a.id
            )
            .execute(&self.pool),
        )?;
        Ok(())
    }

    pub fn event(&self, attempt_id: i64, kind: &str, payload: serde_json::Value) -> Result<()> {
        let at = now();
        let payload = (!payload.is_null()).then(|| payload.to_string());
        self.rt.block_on(
            sqlx::query!(
                "INSERT INTO events (attempt_id, at, kind, payload) VALUES (?, ?, ?, ?)",
                attempt_id,
                at,
                kind,
                payload
            )
            .execute(&self.pool),
        )?;
        Ok(())
    }

    /// Every unfinished attempt, most recent first.
    pub fn open_attempts(&self) -> Result<Vec<OpenAttempt>> {
        Ok(self.rt.block_on(
            sqlx::query_as!(
                OpenAttempt,
                r#"SELECT id AS "id!", question_id, language, started_at, active_secs,
                          test_runs, failed_runs, hints_used,
                          solution_viewed AS "solution_viewed: bool", code
                   FROM attempts
                   WHERE outcome IS NULL
                   ORDER BY id DESC"#
            )
            .fetch_all(&self.pool),
        )?)
    }

    /// The unfinished attempt on a question in a language, if any.
    pub fn open_attempt(&self, question_id: u32, language: &str) -> Result<Option<OpenAttempt>> {
        let question_id = question_id as i64;
        Ok(self.rt.block_on(
            sqlx::query_as!(
                OpenAttempt,
                r#"SELECT id AS "id!", question_id, language, started_at, active_secs,
                          test_runs, failed_runs, hints_used,
                          solution_viewed AS "solution_viewed: bool", code
                   FROM attempts
                   WHERE outcome IS NULL AND question_id = ? AND language = ?
                   ORDER BY id DESC
                   LIMIT 1"#,
                question_id,
                language
            )
            .fetch_optional(&self.pool),
        )?)
    }

    pub fn question_stats(&self, question_id: u32) -> Result<QuestionStats> {
        let question_id = question_id as i64;
        Ok(self.rt.block_on(
            sqlx::query_as!(
                QuestionStats,
                r#"SELECT COUNT(*) AS "attempts!: i64",
                          COALESCE(SUM(outcome IN ('pass', 'revealed')), 0) AS "solved!: i64",
                          MIN(CASE WHEN outcome = 'pass' THEN active_secs END) AS "best_secs: i64",
                          MAX(started_at) AS "last_at: String"
                   FROM attempts
                   WHERE question_id = ? AND outcome IS NOT NULL"#,
                question_id
            )
            .fetch_one(&self.pool),
        )?)
    }
}

pub fn now() -> String {
    jiff::Timestamp::now().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrates_and_records_history() {
        let store = Store::memory().unwrap();
        store.push_history("/show 1").unwrap();
        store.push_history("/list").unwrap();
        store.push_history("/show 1").unwrap();
        assert_eq!(store.history(10).unwrap(), vec!["/list", "/show 1"]);
    }

    #[test]
    fn records_attempts() {
        let store = Store::memory().unwrap();
        let s = store.create_session("id", None).unwrap();
        let a = store.create_attempt(s, 1, "python").unwrap();
        store
            .event(a, "test_run", serde_json::json!({"passed": 1, "total": 3}))
            .unwrap();
        let mut row = AttemptRow {
            id: a,
            active_secs: 300,
            test_runs: 2,
            failed_runs: 1,
            hints_used: 1,
            solution_viewed: false,
            outcome: None,
            code: None,
        };
        store.save_attempt(&row).unwrap();
        assert_eq!(store.question_stats(1).unwrap().attempts, 0);

        row.outcome = Some("pass");
        row.code = Some("def f(): pass");
        store.save_attempt(&row).unwrap();
        let a2 = store.create_attempt(s, 1, "python").unwrap();
        row.id = a2;
        row.active_secs = 200;
        row.outcome = Some("fail");
        store.save_attempt(&row).unwrap();
        store.end_session(s).unwrap();

        assert!(store.open_attempts().unwrap().is_empty());
        let a3 = store.create_attempt(s, 1, "python").unwrap();
        row.id = a3;
        row.active_secs = 42;
        row.outcome = None;
        row.code = Some("wip");
        store.save_attempt(&row).unwrap();
        let open = store.open_attempt(1, "python").unwrap().unwrap();
        assert_eq!(
            (open.id, open.active_secs, open.code.as_deref()),
            (a3, 42, Some("wip"))
        );
        assert!(store.open_attempt(1, "javascript").unwrap().is_none());
        assert_eq!(store.open_attempts().unwrap().len(), 1);

        let stats = store.question_stats(1).unwrap();
        assert_eq!(stats.attempts, 2);
        assert_eq!(stats.solved, 1);
        assert_eq!(stats.best_secs, Some(300));
        assert!(stats.last_at.is_some());
    }
}
