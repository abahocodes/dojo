//! `need`: where practice helps most.

use jiff::Timestamp;

use super::{Strategy, untried_rank};
use crate::model::{GAP, LabelStat, QuestionStat, Suggestion, age_days};

/// In order: a question in each gap topic (practiced, under the mastery
/// bar), due reviews (solved, not cleanly, a week or more ago), a question
/// in each topic never practiced, unsolved questions (ones tried in the last
/// day waiting, so a skip isn't offered straight back), then the weakest
/// solved ones. Untried questions come mediums first.
pub struct Need {
    now: Timestamp,
}

impl Need {
    pub fn new(now: Timestamp) -> Need {
        Need { now }
    }
}

impl Strategy for Need {
    fn pick(&self, pool: &[QuestionStat], topics: &[LabelStat], count: usize) -> Vec<Suggestion> {
        let now = self.now;
        let mut out: Vec<Suggestion> = Vec::new();
        let push = |q: &QuestionStat, reason: String, out: &mut Vec<Suggestion>| {
            if out.len() < count && !out.iter().any(|s| s.question_id == q.id) {
                out.push(
                    Suggestion::builder()
                        .question_id(q.id)
                        .title(q.title.clone())
                        .reason(reason)
                        .build(),
                );
            }
        };
        let untried_first = |q: &&QuestionStat| (untried_rank(q.difficulty), q.id);

        // 1. Gaps: prefer an untried question there, else the weakest one.
        for t in topics.iter().filter(|t| t.attempted > 0 && t.mastery < GAP) {
            let pick = in_topic(pool, &t.label)
                .filter(|q| q.attempts == 0)
                .min_by_key(untried_first)
                .or_else(|| {
                    in_topic(pool, &t.label)
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
        // 2. Due reviews.
        let mut due: Vec<&QuestionStat> = pool
            .iter()
            .filter(|q| q.solved && q.score.unwrap_or(1.0) < 0.85)
            .filter(|q| q.last_attempt.is_some_and(|t| age_days(t, now) >= 7.0))
            .collect();
        due.sort_by(|a, b| a.score.unwrap_or(0.0).total_cmp(&b.score.unwrap_or(0.0)));
        for q in due {
            push(q, "due for review".into(), &mut out);
        }
        // 3. Coverage.
        for t in topics.iter().filter(|t| t.attempted == 0) {
            if let Some(q) = in_topic(pool, &t.label).min_by_key(untried_first) {
                push(q, format!("{} not practiced yet", t.label), &mut out);
            }
        }
        // 4. Unsolved.
        let mut rest: Vec<&QuestionStat> = pool.iter().filter(|q| !q.solved).collect();
        rest.sort_by_key(|q| {
            let recent = q.last_attempt.is_some_and(|t| age_days(t, now) < 1.0);
            (recent, untried_rank(q.difficulty), q.id)
        });
        for q in rest {
            let reason = if q.attempts == 0 {
                "not tried yet"
            } else {
                "not solved yet"
            };
            push(q, reason.into(), &mut out);
        }
        // 5. Weakest solved.
        let mut weakest: Vec<&QuestionStat> = pool.iter().filter(|q| q.solved).collect();
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
}

fn in_topic<'a>(pool: &'a [QuestionStat], t: &'a str) -> impl Iterator<Item = &'a QuestionStat> {
    pool.iter().filter(move |q| q.tags.iter().any(|x| x == t))
}
