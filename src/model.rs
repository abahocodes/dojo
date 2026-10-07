//! The competence model: grades attempts and rolls them up into question
//! scores, topic mastery, company readiness and suggestions. Pure functions
//! over recorded attempts, so the model can change without migrations.

use std::collections::{BTreeMap, BTreeSet};

use jiff::civil::Date;
use jiff::tz::TimeZone;
use jiff::{Timestamp, ToSpan};
use serde::Serialize;

use crate::questions::{Bank, Difficulty, Question};

/// Older attempts count less: a grade's weight halves every this many days.
const HALF_LIFE_DAYS: f64 = 30.0;
/// Topic mastery below this is a gap.
pub const GAP: f64 = 0.6;

/// One recorded attempt, as the model sees it.
#[derive(Debug, Clone)]
pub struct Attempt {
    pub session_id: i64,
    pub question_id: u32,
    pub started_at: Timestamp,
    pub active_secs: u64,
    /// `None` while unfinished.
    pub outcome: Option<String>,
    pub failed_runs: u32,
    pub hints_used: u32,
}

impl Attempt {
    /// From a stored row; `None` if its timestamp doesn't parse.
    pub fn from_record(r: &crate::store::AttemptRecord) -> Option<Attempt> {
        Some(Attempt {
            session_id: r.session_id,
            question_id: u32::try_from(r.question_id).ok()?,
            started_at: r.started_at.parse().ok()?,
            active_secs: r.active_secs.max(0) as u64,
            outcome: r.outcome.clone(),
            failed_runs: r.failed_runs.max(0) as u32,
            hints_used: r.hints_used.max(0) as u32,
        })
    }

    pub fn solved(&self) -> bool {
        matches!(self.outcome.as_deref(), Some("pass" | "revealed"))
    }
}

/// 0–1: how well an attempt went. A clean pass inside the target time is 1.
pub fn grade(a: &Attempt, q: &Question) -> f64 {
    match a.outcome.as_deref() {
        Some("pass") => {
            let mut g = 1.0;
            g -= 0.08 * a.failed_runs.min(4) as f64;
            if !q.hints.is_empty() {
                g -= 0.4 * (a.hints_used as f64 / q.hints.len() as f64).min(1.0);
            }
            let target = (q.meta.target_minutes as f64 * 60.0).max(1.0);
            let over = a.active_secs as f64 / target - 1.0;
            if over > 0.0 {
                g -= 0.25 * over.min(1.0);
            }
            g.max(0.35)
        }
        Some("revealed") => 0.15,
        _ => 0.0,
    }
}

fn difficulty_weight(d: Difficulty) -> f64 {
    match d {
        Difficulty::Easy => 0.8,
        Difficulty::Medium => 1.0,
        Difficulty::Hard => 1.2,
    }
}

fn age_days(then: Timestamp, now: Timestamp) -> f64 {
    ((now.as_second() - then.as_second()).max(0) as f64) / 86_400.0
}

#[derive(Debug, Clone, Serialize)]
pub struct QuestionStat {
    pub id: u32,
    pub title: String,
    pub difficulty: Difficulty,
    pub tags: Vec<String>,
    pub companies: Vec<String>,
    /// Finished attempts.
    pub attempts: u32,
    pub solved: bool,
    /// Recency-weighted grade, `None` when never attempted.
    pub score: Option<f64>,
    /// Fastest clean pass, in seconds.
    pub best_secs: Option<u64>,
    pub target_secs: u64,
    pub last_attempt: Option<Timestamp>,
    pub last_outcome: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LabelStat {
    pub label: String,
    /// Topics: mean score over attempted questions. Companies: mean over
    /// all questions, unattempted counting as 0.
    pub mastery: f64,
    pub attempted: usize,
    pub total: usize,
    /// 0 = little evidence, 1 = some, 2 = solid.
    pub confidence: u8,
}

#[derive(Debug, Clone, Serialize)]
pub struct DifficultyStat {
    pub difficulty: Difficulty,
    pub solved: usize,
    pub total: usize,
    /// Mean of time / target over passes.
    pub time_ratio: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Overview {
    pub questions: usize,
    pub solved: usize,
    pub attempted: usize,
    pub attempts: usize,
    pub pass_rate: Option<f64>,
    pub practice_secs: u64,
    pub streak_days: u32,
    pub difficulties: Vec<DifficultyStat>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Suggestion {
    pub question_id: u32,
    pub title: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SessionAttempt {
    pub question_id: u32,
    pub title: String,
    pub outcome: Option<String>,
    pub active_secs: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SessionStat {
    pub id: i64,
    pub started_at: Timestamp,
    pub practice_secs: u64,
    pub attempts: Vec<SessionAttempt>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Report {
    pub generated_at: Timestamp,
    pub overview: Overview,
    /// Weakest first; unpracticed topics last.
    pub topics: Vec<LabelStat>,
    /// Least ready first.
    pub companies: Vec<LabelStat>,
    pub questions: Vec<QuestionStat>,
    /// Most recent first.
    pub sessions: Vec<SessionStat>,
    /// Attempts per local day, oldest first, for the last `ACTIVITY_DAYS`.
    pub activity: Vec<(Date, u32)>,
    pub suggestions: Vec<Suggestion>,
}

pub const ACTIVITY_DAYS: i64 = 7 * 18;

pub fn build(bank: &Bank, attempts: &[Attempt], now: Timestamp, tz: &TimeZone) -> Report {
    let by_question: BTreeMap<u32, Vec<&Attempt>> =
        attempts.iter().fold(BTreeMap::new(), |mut m, a| {
            m.entry(a.question_id).or_insert_with(Vec::new).push(a);
            m
        });

    let questions: Vec<QuestionStat> = bank
        .all()
        .iter()
        .map(|q| question_stat(q, by_question.get(&q.meta.id).map(Vec::as_slice), now))
        .collect();

    let finished: Vec<&Attempt> = attempts.iter().filter(|a| a.outcome.is_some()).collect();
    let overview = Overview {
        questions: questions.len(),
        solved: questions.iter().filter(|q| q.solved).count(),
        attempted: questions.iter().filter(|q| q.attempts > 0).count(),
        attempts: finished.len(),
        pass_rate: (!finished.is_empty())
            .then(|| finished.iter().filter(|a| a.solved()).count() as f64 / finished.len() as f64),
        practice_secs: attempts.iter().map(|a| a.active_secs).sum(),
        streak_days: streak(attempts, now, tz),
        difficulties: [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard]
            .into_iter()
            .map(|d| difficulty_stat(d, &questions, &finished, bank))
            .collect(),
    };

    let topics = label_stats(&questions, |q| &q.tags, false);
    let companies = label_stats(&questions, |q| &q.companies, true);
    let suggestions = suggest(&questions, &topics, now, 3);

    Report {
        generated_at: now,
        overview,
        topics,
        companies,
        sessions: sessions(bank, attempts),
        activity: activity(attempts, now, tz),
        suggestions,
        questions,
    }
}

fn question_stat(q: &Question, attempts: Option<&[&Attempt]>, now: Timestamp) -> QuestionStat {
    let finished: Vec<&&Attempt> = attempts
        .unwrap_or_default()
        .iter()
        .filter(|a| a.outcome.is_some())
        .collect();
    let mut weight = 0.0;
    let mut total = 0.0;
    for a in &finished {
        // Skipping without trying is weaker evidence than a real attempt.
        let tried = if a.outcome.as_deref() == Some("skip") {
            0.5
        } else {
            1.0
        };
        let w = tried * 0.5_f64.powf(age_days(a.started_at, now) / HALF_LIFE_DAYS);
        weight += w;
        total += w * grade(a, q);
    }
    let last = finished.iter().max_by_key(|a| a.started_at);
    QuestionStat {
        id: q.meta.id,
        title: q.meta.title.clone(),
        difficulty: q.meta.difficulty,
        tags: q.meta.tags.clone(),
        companies: q.meta.companies.clone(),
        attempts: finished.len() as u32,
        solved: finished.iter().any(|a| a.solved()),
        score: (weight > 0.0).then(|| total / weight),
        best_secs: finished
            .iter()
            .filter(|a| a.outcome.as_deref() == Some("pass"))
            .map(|a| a.active_secs)
            .min(),
        target_secs: q.meta.target_minutes as u64 * 60,
        last_attempt: last.map(|a| a.started_at),
        last_outcome: last.and_then(|a| a.outcome.clone()),
    }
}

fn label_stats<'a>(
    questions: &'a [QuestionStat],
    labels: impl Fn(&'a QuestionStat) -> &'a Vec<String>,
    unattempted_count: bool,
) -> Vec<LabelStat> {
    let mut groups: BTreeMap<&str, Vec<&QuestionStat>> = BTreeMap::new();
    for q in questions {
        for label in labels(q) {
            groups.entry(label).or_default().push(q);
        }
    }
    let mut out: Vec<LabelStat> = groups
        .into_iter()
        .map(|(label, qs)| {
            let attempted: Vec<&&QuestionStat> = qs.iter().filter(|q| q.score.is_some()).collect();
            let pool: Vec<(f64, f64)> = if unattempted_count {
                qs.iter()
                    .map(|q| (q.score.unwrap_or(0.0), difficulty_weight(q.difficulty)))
                    .collect()
            } else {
                attempted
                    .iter()
                    .map(|q| (q.score.unwrap_or(0.0), difficulty_weight(q.difficulty)))
                    .collect()
            };
            let w: f64 = pool.iter().map(|(_, w)| w).sum();
            LabelStat {
                label: label.to_string(),
                mastery: if w > 0.0 {
                    pool.iter().map(|(s, w)| s * w).sum::<f64>() / w
                } else {
                    0.0
                },
                attempted: attempted.len(),
                total: qs.len(),
                confidence: match attempted.len() {
                    0 | 1 => 0,
                    2 | 3 => 1,
                    _ => 2,
                },
            }
        })
        .collect();
    // Weakest first; never-practiced labels go last.
    out.sort_by(|a, b| {
        (a.attempted == 0)
            .cmp(&(b.attempted == 0))
            .then(a.mastery.total_cmp(&b.mastery))
            .then(a.label.cmp(&b.label))
    });
    out
}

fn difficulty_stat(
    d: Difficulty,
    questions: &[QuestionStat],
    finished: &[&Attempt],
    bank: &Bank,
) -> DifficultyStat {
    let ratios: Vec<f64> = finished
        .iter()
        .filter(|a| a.outcome.as_deref() == Some("pass"))
        .filter_map(|a| {
            let q = bank.get(a.question_id)?;
            (q.meta.difficulty == d)
                .then(|| a.active_secs as f64 / (q.meta.target_minutes as f64 * 60.0).max(1.0))
        })
        .collect();
    DifficultyStat {
        difficulty: d,
        solved: questions
            .iter()
            .filter(|q| q.difficulty == d && q.solved)
            .count(),
        total: questions.iter().filter(|q| q.difficulty == d).count(),
        time_ratio: (!ratios.is_empty()).then(|| ratios.iter().sum::<f64>() / ratios.len() as f64),
    }
}

fn local_date(t: Timestamp, tz: &TimeZone) -> Date {
    t.to_zoned(tz.clone()).date()
}

/// Consecutive days, ending today or yesterday, with at least one attempt.
fn streak(attempts: &[Attempt], now: Timestamp, tz: &TimeZone) -> u32 {
    let days: BTreeSet<Date> = attempts
        .iter()
        .map(|a| local_date(a.started_at, tz))
        .collect();
    let today = local_date(now, tz);
    let mut day = if days.contains(&today) {
        today
    } else {
        match today.checked_sub(1.day()) {
            Ok(d) if days.contains(&d) => d,
            _ => return 0,
        }
    };
    let mut n = 0;
    while days.contains(&day) {
        n += 1;
        match day.checked_sub(1.day()) {
            Ok(d) => day = d,
            Err(_) => break,
        }
    }
    n
}

fn activity(attempts: &[Attempt], now: Timestamp, tz: &TimeZone) -> Vec<(Date, u32)> {
    let today = local_date(now, tz);
    let mut counts: BTreeMap<Date, u32> = BTreeMap::new();
    for a in attempts {
        *counts.entry(local_date(a.started_at, tz)).or_default() += 1;
    }
    (0..ACTIVITY_DAYS)
        .rev()
        .filter_map(|back| today.checked_sub(back.days()).ok())
        .map(|d| (d, counts.get(&d).copied().unwrap_or(0)))
        .collect()
}

fn sessions(bank: &Bank, attempts: &[Attempt]) -> Vec<SessionStat> {
    let mut by_session: BTreeMap<i64, Vec<&Attempt>> = BTreeMap::new();
    for a in attempts {
        by_session.entry(a.session_id).or_default().push(a);
    }
    let mut out: Vec<SessionStat> = by_session
        .into_iter()
        .map(|(id, list)| SessionStat {
            id,
            started_at: list.iter().map(|a| a.started_at).min().unwrap_or_default(),
            practice_secs: list.iter().map(|a| a.active_secs).sum(),
            attempts: list
                .iter()
                .map(|a| SessionAttempt {
                    question_id: a.question_id,
                    title: bank
                        .get(a.question_id)
                        .map(|q| q.meta.title.clone())
                        .unwrap_or_default(),
                    outcome: a.outcome.clone(),
                    active_secs: a.active_secs,
                })
                .collect(),
        })
        .collect();
    out.sort_by(|a, b| b.started_at.cmp(&a.started_at));
    out
}

/// What to practice next: weak topics first, then due reviews, then topics
/// never practiced. Up to three, each with a reason.
pub fn suggest(
    questions: &[QuestionStat],
    topics: &[LabelStat],
    now: Timestamp,
    limit: usize,
) -> Vec<Suggestion> {
    let mut out: Vec<Suggestion> = Vec::new();
    let push = |q: &QuestionStat, reason: String, out: &mut Vec<Suggestion>| {
        if out.len() < limit && !out.iter().any(|s| s.question_id == q.id) {
            out.push(Suggestion {
                question_id: q.id,
                title: q.title.clone(),
                reason,
            });
        }
    };
    fn in_topic<'a>(qs: &'a [QuestionStat], t: &'a str) -> impl Iterator<Item = &'a QuestionStat> {
        qs.iter().filter(move |q| q.tags.iter().any(|x| x == t))
    }

    // 1. Gaps: practiced topics below the mastery bar. Prefer an untried
    //    question there, else the weakest one.
    for t in topics.iter().filter(|t| t.attempted > 0 && t.mastery < GAP) {
        let pick = in_topic(questions, &t.label)
            .filter(|q| q.attempts == 0)
            .min_by_key(|q| q.difficulty)
            .or_else(|| {
                in_topic(questions, &t.label)
                    .min_by(|a, b| a.score.unwrap_or(0.0).total_cmp(&b.score.unwrap_or(0.0)))
            });
        if let Some(q) = pick {
            push(
                q,
                format!("{} is a gap ({:.0}% mastery)", t.label, t.mastery * 100.0),
                &mut out,
            );
        }
    }
    // 2. Due reviews: solved a while ago, not cleanly.
    let mut due: Vec<&QuestionStat> = questions
        .iter()
        .filter(|q| q.solved && q.score.unwrap_or(1.0) < 0.85)
        .filter(|q| q.last_attempt.is_some_and(|t| age_days(t, now) >= 7.0))
        .collect();
    due.sort_by(|a, b| a.score.unwrap_or(0.0).total_cmp(&b.score.unwrap_or(0.0)));
    for q in due {
        push(q, "due for review".into(), &mut out);
    }
    // 3. Coverage: topics never practiced, easiest question first.
    for t in topics.iter().filter(|t| t.attempted == 0) {
        if let Some(q) = in_topic(questions, &t.label).min_by_key(|q| q.difficulty) {
            push(q, format!("{} not practiced yet", t.label), &mut out);
        }
    }
    // 4. Still short (asked for many): unsolved questions, easiest first,
    //    then the weakest solved ones.
    let mut rest: Vec<&QuestionStat> = questions.iter().filter(|q| !q.solved).collect();
    rest.sort_by_key(|q| (q.difficulty, q.id));
    for q in rest {
        let reason = if q.attempts == 0 {
            "not tried yet"
        } else {
            "not solved yet"
        };
        push(q, reason.into(), &mut out);
    }
    let mut weakest: Vec<&QuestionStat> = questions.iter().filter(|q| q.solved).collect();
    weakest.sort_by(|a, b| a.score.unwrap_or(0.0).total_cmp(&b.score.unwrap_or(0.0)));
    for q in weakest {
        push(
            q,
            format!("weakest solved ({:.0}%)", q.score.unwrap_or(0.0) * 100.0),
            &mut out,
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(days_ago: i64) -> Timestamp {
        let now: Timestamp = "2026-10-07T12:00:00Z".parse().unwrap();
        now.checked_sub((days_ago * 24).hours()).unwrap()
    }

    fn attempt(q: u32, days_ago: i64, outcome: &str, secs: u64) -> Attempt {
        Attempt {
            session_id: 1,
            question_id: q,
            started_at: at(days_ago),
            active_secs: secs,
            outcome: Some(outcome.into()),
            failed_runs: 0,
            hints_used: 0,
        }
    }

    #[test]
    fn grades_attempts() {
        let bank = Bank::embedded();
        let q = bank.get(1).unwrap(); // 15 min target, 3 hints
        assert_eq!(grade(&attempt(1, 0, "pass", 300), q), 1.0);
        let mut slow = attempt(1, 0, "pass", 30 * 60);
        assert!((grade(&slow, q) - 0.75).abs() < 1e-9); // double the target
        slow.hints_used = 3;
        slow.failed_runs = 10;
        assert_eq!(grade(&slow, q), 0.35); // floor for a pass
        assert_eq!(grade(&attempt(1, 0, "revealed", 60), q), 0.15);
        assert_eq!(grade(&attempt(1, 0, "fail", 60), q), 0.0);
    }

    #[test]
    fn builds_report() {
        let bank = Bank::embedded();
        let now = at(0);
        let attempts = vec![
            attempt(1, 0, "pass", 300),  // arrays, hash-map
            attempt(11, 1, "fail", 900), // graphs ...
            attempt(11, 0, "skip", 30),
            attempt(13, 2, "pass", 200),
        ];
        let r = build(&bank, &attempts, now, &TimeZone::UTC);
        assert_eq!(r.overview.solved, 2);
        assert_eq!(r.overview.attempted, 3);
        assert_eq!(r.overview.streak_days, 3);
        assert_eq!(r.overview.practice_secs, 1430);
        let graphs = r.topics.iter().find(|t| t.label == "graphs").unwrap();
        assert_eq!(graphs.mastery, 0.0);
        // Weakest practiced topic first, unpracticed last.
        assert!(r.topics.first().unwrap().attempted > 0);
        assert_eq!(r.topics.last().unwrap().attempted, 0);
        assert!(r.suggestions.iter().any(|s| s.reason.contains("graphs")));
        assert_eq!(r.activity.len(), ACTIVITY_DAYS as usize);
        assert_eq!(r.activity.last().unwrap().1, 2);
        assert_eq!(r.sessions.len(), 1);
        // Asking for more than the gaps fills up with untried questions.
        let many = suggest(&r.questions, &r.topics, now, 10);
        assert_eq!(many.len(), 10);
        assert_eq!(many[0].question_id, r.suggestions[0].question_id);
        let all = suggest(&r.questions, &r.topics, now, 99);
        assert_eq!(all.len(), bank.all().len());
    }

    #[test]
    fn older_attempts_count_less() {
        let bank = Bank::embedded();
        let q = bank.get(1).unwrap();
        let recent_fail = [attempt(1, 0, "fail", 60), attempt(1, 90, "pass", 60)];
        let refs: Vec<&Attempt> = recent_fail.iter().collect();
        let s = question_stat(q, Some(&refs), at(0)).score.unwrap();
        assert!(s < 0.2, "recent fail should dominate, got {s}");
    }
}
