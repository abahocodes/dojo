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
use jiff::Timestamp;
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::types::time::OffsetDateTime;
use tokio::runtime::Runtime;

use crate::lang::Language;
use crate::session::Outcome;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

pub struct Store {
    pool: SqlitePool,
    rt: Runtime,
}

/// Attempt counters persisted after every change, so a crash loses nothing.
#[derive(bon::Builder)]
pub struct AttemptRow<'a> {
    pub id: i64,
    pub active_secs: u64,
    pub test_runs: u32,
    pub failed_runs: u32,
    pub hints_used: usize,
    pub solution_viewed: bool,
    /// Set when the attempt finishes (which also sets its end time).
    pub outcome: Option<Outcome>,
    pub code: Option<&'a str>,
}

/// What happened during an attempt (`events.kind`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
pub enum EventKind {
    Start,
    Continue,
    EditorOpen,
    TestRun,
    Hint,
    SolutionViewed,
    Pause,
    IdlePause,
    Resume,
    Suspend,
    Outcome,
}

/// What the user has done on one question before.
#[derive(Debug, Default)]
pub struct QuestionStats {
    pub attempts: i64,
    pub solved: i64,
    pub best_secs: Option<i64>,
    pub last_at: Option<Timestamp>,
}

/// An attempt that was started but never finished; it can be continued.
#[derive(Debug, Clone)]
pub struct OpenAttempt {
    pub id: i64,
    pub question_id: i64,
    pub language: Language,
    pub started_at: Timestamp,
    pub active_secs: i64,
    pub test_runs: i64,
    pub failed_runs: i64,
    pub hints_used: i64,
    pub solution_viewed: bool,
    pub code: Option<String>,
}

/// Every attempt, for the report.
#[derive(Debug, Clone)]
pub struct AttemptRecord {
    pub session_id: i64,
    pub question_id: i64,
    pub started_at: Timestamp,
    pub active_secs: i64,
    pub outcome: Option<Outcome>,
    pub failed_runs: i64,
    pub hints_used: i64,
}

/// One past attempt on a question, with its code.
#[derive(Debug, Clone)]
pub struct PastAttempt {
    pub language: Language,
    pub started_at: Timestamp,
    pub active_secs: i64,
    pub outcome: Option<Outcome>,
    pub test_runs: i64,
    pub hints_used: i64,
    pub code: Option<String>,
}

struct Id {
    id: i64,
}

struct HistoryLine {
    line: String,
}

/// Rows as SQLite returns them; timestamps become `jiff::Timestamp` (what
/// the rest of dojo uses) at this boundary.
struct OpenRow {
    id: i64,
    question_id: i64,
    language: Language,
    started_at: OffsetDateTime,
    active_secs: i64,
    test_runs: i64,
    failed_runs: i64,
    hints_used: i64,
    solution_viewed: bool,
    code: Option<String>,
}

impl From<OpenRow> for OpenAttempt {
    fn from(r: OpenRow) -> OpenAttempt {
        OpenAttempt {
            id: r.id,
            question_id: r.question_id,
            language: r.language,
            started_at: to_jiff(r.started_at),
            active_secs: r.active_secs,
            test_runs: r.test_runs,
            failed_runs: r.failed_runs,
            hints_used: r.hints_used,
            solution_viewed: r.solution_viewed,
            code: r.code,
        }
    }
}

struct RecordRow {
    session_id: i64,
    question_id: i64,
    started_at: OffsetDateTime,
    active_secs: i64,
    outcome: Option<Outcome>,
    failed_runs: i64,
    hints_used: i64,
}

struct PastRow {
    language: Language,
    started_at: OffsetDateTime,
    active_secs: i64,
    outcome: Option<Outcome>,
    test_runs: i64,
    hints_used: i64,
    code: Option<String>,
}

struct StatsRow {
    attempts: i64,
    solved: i64,
    best_secs: Option<i64>,
    last_at: Option<OffsetDateTime>,
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
        rt.block_on(MIGRATOR.run(&pool)).map_err(|e| match e {
            // The schema was rewritten before dojo's first users; a database
            // from a pre-release build can't be migrated.
            sqlx::migrate::MigrateError::VersionMismatch(_)
            | sqlx::migrate::MigrateError::VersionMissing(_) => anyhow::anyhow!(
                "this database was created by a pre-release dojo with an older schema; \
                 move it aside (or delete it) and dojo will start a fresh one"
            ),
            other => anyhow::Error::new(other).context(
                "migrating the database (if it was created by a newer dojo, upgrade dojo)",
            ),
        })?;
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

    pub fn create_session(&self) -> Result<i64> {
        let at = now();
        let row = self.rt.block_on(
            sqlx::query_as!(
                Id,
                r#"INSERT INTO sessions (started_at) VALUES (?) RETURNING id AS "id!""#,
                at
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

    pub fn create_attempt(
        &self,
        session_id: i64,
        question_id: u32,
        language: Language,
    ) -> Result<i64> {
        let at = now();
        let question_id = question_id as i64;
        let row = self.rt.block_on(
            sqlx::query_as!(
                Id,
                r#"INSERT INTO attempts (session_id, question_id, language, started_at)
                   VALUES (?, ?, ?, ?)
                   RETURNING id AS "id!""#,
                session_id,
                question_id,
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

    pub fn event(
        &self,
        attempt_id: i64,
        kind: EventKind,
        payload: serde_json::Value,
    ) -> Result<()> {
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
        let rows = self.rt.block_on(
            sqlx::query_as!(
                OpenRow,
                r#"SELECT id AS "id!", question_id, language AS "language: Language",
                          started_at AS "started_at: OffsetDateTime", active_secs,
                          test_runs, failed_runs, hints_used,
                          solution_viewed AS "solution_viewed: bool", code
                   FROM attempts
                   WHERE outcome IS NULL
                   ORDER BY id DESC"#
            )
            .fetch_all(&self.pool),
        )?;
        Ok(rows.into_iter().map(OpenAttempt::from).collect())
    }

    /// The unfinished attempt on a question in a language, if any.
    pub fn open_attempt(
        &self,
        question_id: u32,
        language: Language,
    ) -> Result<Option<OpenAttempt>> {
        let question_id = question_id as i64;
        let row = self.rt.block_on(
            sqlx::query_as!(
                OpenRow,
                r#"SELECT id AS "id!", question_id, language AS "language: Language",
                          started_at AS "started_at: OffsetDateTime", active_secs,
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
        )?;
        Ok(row.map(OpenAttempt::from))
    }

    /// Every attempt on one question, oldest first.
    pub fn past_attempts(&self, question_id: u32) -> Result<Vec<PastAttempt>> {
        let question_id = question_id as i64;
        let rows = self.rt.block_on(
            sqlx::query_as!(
                PastRow,
                r#"SELECT language AS "language: Language",
                          started_at AS "started_at: OffsetDateTime", active_secs,
                          outcome AS "outcome: Outcome", test_runs, hints_used, code
                   FROM attempts
                   WHERE question_id = ?
                   ORDER BY id"#,
                question_id
            )
            .fetch_all(&self.pool),
        )?;
        Ok(rows
            .into_iter()
            .map(|r| PastAttempt {
                language: r.language,
                started_at: to_jiff(r.started_at),
                active_secs: r.active_secs,
                outcome: r.outcome,
                test_runs: r.test_runs,
                hints_used: r.hints_used,
                code: r.code,
            })
            .collect())
    }

    /// Every attempt, oldest first.
    pub fn attempts(&self) -> Result<Vec<AttemptRecord>> {
        let rows = self.rt.block_on(
            sqlx::query_as!(
                RecordRow,
                r#"SELECT session_id, question_id, started_at AS "started_at: OffsetDateTime",
                          active_secs, outcome AS "outcome: Outcome", failed_runs, hints_used
                   FROM attempts
                   ORDER BY id"#
            )
            .fetch_all(&self.pool),
        )?;
        Ok(rows
            .into_iter()
            .map(|r| AttemptRecord {
                session_id: r.session_id,
                question_id: r.question_id,
                started_at: to_jiff(r.started_at),
                active_secs: r.active_secs,
                outcome: r.outcome,
                failed_runs: r.failed_runs,
                hints_used: r.hints_used,
            })
            .collect())
    }

    pub fn question_stats(&self, question_id: u32) -> Result<QuestionStats> {
        let question_id = question_id as i64;
        let r = self.rt.block_on(
            sqlx::query_as!(
                StatsRow,
                r#"SELECT COUNT(*) AS "attempts!: i64",
                          COALESCE(SUM(outcome IN ('pass', 'revealed')), 0) AS "solved!: i64",
                          MIN(CASE WHEN outcome = 'pass' THEN active_secs END) AS "best_secs: i64",
                          MAX(started_at) AS "last_at: OffsetDateTime"
                   FROM attempts
                   WHERE question_id = ? AND outcome IS NOT NULL"#,
                question_id
            )
            .fetch_one(&self.pool),
        )?;
        Ok(QuestionStats {
            attempts: r.attempts,
            solved: r.solved,
            best_secs: r.best_secs,
            last_at: r.last_at.map(to_jiff),
        })
    }
}

/// The current time, as stored.
fn now() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

fn to_jiff(t: OffsetDateTime) -> Timestamp {
    Timestamp::from_nanosecond(t.unix_timestamp_nanos()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The schema rejects what the app should never write.
    #[rstest::rstest]
    #[case::unknown_language("UPDATE attempts SET language = 'rust'")]
    #[case::unknown_outcome("UPDATE attempts SET outcome = 'won', ended_at = started_at")]
    #[case::outcome_without_end("UPDATE attempts SET outcome = 'pass'")]
    #[case::end_without_outcome("UPDATE attempts SET ended_at = started_at")]
    #[case::more_failures_than_runs("UPDATE attempts SET test_runs = 1, failed_runs = 2")]
    #[case::negative_time("UPDATE attempts SET active_secs = -1")]
    #[case::ends_before_it_starts(
        "UPDATE attempts SET outcome = 'pass', ended_at = '2000-01-01 00:00:00'"
    )]
    #[case::unknown_event(
        "INSERT INTO events (attempt_id, at, kind) VALUES (1, '2026-01-01 00:00:00', 'teleport')"
    )]
    #[case::payload_not_json(
        "INSERT INTO events (attempt_id, at, kind, payload) VALUES (1, '2026-01-01 00:00:00', 'hint', '{oops')"
    )]
    #[case::unknown_attempt(
        "INSERT INTO events (attempt_id, at, kind) VALUES (99, '2026-01-01 00:00:00', 'hint')"
    )]
    #[case::empty_history_line(
        "INSERT INTO input_history (at, line) VALUES ('2026-01-01 00:00:00', '')"
    )]
    fn schema_rejects_bad_rows(#[case] sql: &'static str) {
        let store = Store::memory().unwrap();
        let s = store.create_session().unwrap();
        store.create_attempt(s, 1, Language::Python).unwrap();
        let result = store.rt.block_on(sqlx::query(sql).execute(&store.pool));
        assert!(result.is_err(), "{sql} was accepted");
    }

    #[test]
    fn timestamps_round_trip() {
        let store = Store::memory().unwrap();
        let before = Timestamp::now();
        let s = store.create_session().unwrap();
        store.create_attempt(s, 7, Language::Go).unwrap();
        let open = store.open_attempt(7, Language::Go).unwrap().unwrap();
        let after = Timestamp::now();
        assert!(before <= open.started_at && open.started_at <= after);
        assert_eq!(open.language, Language::Go);
    }

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
        let s = store.create_session().unwrap();
        let a = store.create_attempt(s, 1, Language::Python).unwrap();
        store
            .event(
                a,
                EventKind::TestRun,
                serde_json::json!({"passed": 1, "total": 3}),
            )
            .unwrap();
        let mut row = AttemptRow::builder()
            .id(a)
            .active_secs(300)
            .test_runs(2)
            .failed_runs(1)
            .hints_used(1)
            .solution_viewed(false)
            .build();
        store.save_attempt(&row).unwrap();
        assert_eq!(store.question_stats(1).unwrap().attempts, 0);

        row.outcome = Some(Outcome::Pass);
        row.code = Some("def f(): pass");
        store.save_attempt(&row).unwrap();
        let a2 = store.create_attempt(s, 1, Language::Python).unwrap();
        row.id = a2;
        row.active_secs = 200;
        row.outcome = Some(Outcome::Fail);
        store.save_attempt(&row).unwrap();
        store.end_session(s).unwrap();

        assert!(store.open_attempts().unwrap().is_empty());
        let a3 = store.create_attempt(s, 1, Language::Python).unwrap();
        row.id = a3;
        row.active_secs = 42;
        row.outcome = None;
        row.code = Some("wip");
        store.save_attempt(&row).unwrap();
        let open = store.open_attempt(1, Language::Python).unwrap().unwrap();
        assert_eq!(
            (open.id, open.active_secs, open.code.as_deref()),
            (a3, 42, Some("wip"))
        );
        assert!(
            store
                .open_attempt(1, Language::JavaScript)
                .unwrap()
                .is_none()
        );
        assert_eq!(store.open_attempts().unwrap().len(), 1);
        assert_eq!(store.attempts().unwrap().len(), 3);
        let past = store.past_attempts(1).unwrap();
        assert_eq!(past.len(), 3);
        assert_eq!(past[0].code.as_deref(), Some("def f(): pass"));

        let stats = store.question_stats(1).unwrap();
        assert_eq!(stats.attempts, 2);
        assert_eq!(stats.solved, 1);
        assert_eq!(stats.best_secs, Some(300));
        assert!(stats.last_at.is_some());
    }
}
