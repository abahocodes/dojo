//! Choosing what to practice. `/solve` words parse into a [`Request`]
//! regardless of order (`google need 3` is `need google 3`), and a
//! [`Strategy`] picks from the questions a [`Filter`] lets through.
//! Questions and history come from a [`Library`] and chance from [`Dice`],
//! so selection is pure and testable with mocks.

mod need;
mod random;

use jiff::Timestamp;

use crate::model::{LabelStat, QuestionStat, Suggestion};
use crate::questions::{Difficulty, Question};

pub use need::Need;
pub use random::{Clock, Random};

/// Where questions and your history come from.
pub trait Library {
    /// Every tag and company name, lowercase.
    fn labels(&self) -> Vec<String>;
    /// The questions you can practice now (in your language) with your
    /// history on each, and your mastery per topic.
    fn history(&self) -> anyhow::Result<(Vec<QuestionStat>, Vec<LabelStat>)>;
    /// Ids matching free text, best first; an exact slug alone.
    fn search(&self, text: &str) -> Vec<u32>;
}

/// A source of chance.
pub trait Dice {
    /// A number in `0..n` (`n > 0`).
    fn below(&self, n: usize) -> usize;
}

/// A way to order the questions a filter lets through.
pub trait Strategy {
    /// Up to `count` questions from `pool`, each with why it was picked.
    fn pick(&self, pool: &[QuestionStat], topics: &[LabelStat], count: usize) -> Vec<Suggestion>;
}

/// The strategies `/solve` can name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Where you need practice; the default.
    Need,
    Random,
}

impl Kind {
    fn parse(word: &str) -> Option<Kind> {
        match word {
            "need" => Some(Kind::Need),
            "random" => Some(Kind::Random),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Kind::Need => "need",
            Kind::Random => "random",
        }
    }
}

/// What `/solve` asked for.
#[derive(Debug, PartialEq)]
pub enum Request {
    /// `/solve 3 7`: these questions, in this order.
    Ids(Vec<u32>),
    /// `/solve two sum`: the best title matches.
    Search { text: String, count: usize },
    /// `/solve need google 3`, `/solve dfs easy`, `/solve random`.
    Pick {
        kind: Kind,
        filter: Filter,
        count: usize,
    },
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseError {
    #[error("what should we practice?  ·  /solve 12, /solve need, /solve google dfs 2")]
    Empty,
    #[error("-n needs a positive number")]
    BadN,
    #[error("`{0}` isn't a question id  ·  /list")]
    BadId(String),
    #[error("`{0}` isn't a count; give a positive number")]
    BadCount(String),
    #[error("give the count once: `3` or `-n 3`")]
    TwoCounts,
    #[error("pick one of need or random")]
    TwoKinds,
    #[error("`{word}` isn't a tag, company or difficulty  ·  /solve {kind} google dfs 2")]
    NotAFilter { word: String, kind: &'static str },
}

impl Request {
    /// `/solve` arguments, given the known tag and company names.
    pub fn parse(args: &[&str], labels: &[String]) -> Result<Request, ParseError> {
        let mut n: Option<usize> = None;
        let mut words: Vec<&str> = Vec::new();
        let mut it = args.iter();
        while let Some(&a) = it.next() {
            if a == "-n" {
                match it.next().and_then(|v| v.parse().ok()) {
                    Some(v) if v > 0 => n = Some(v),
                    _ => return Err(ParseError::BadN),
                }
            } else {
                words.push(a);
            }
        }
        if words.is_empty() {
            return Err(ParseError::Empty);
        }

        // Only numbers: ids (`/solve 3 3` practices #3 once).
        if words.iter().all(|w| looks_numeric(w)) {
            let mut ids: Vec<u32> = Vec::new();
            for w in &words {
                let id = w.parse().map_err(|_| ParseError::BadId(w.to_string()))?;
                if !ids.contains(&id) {
                    ids.push(id);
                }
            }
            return Ok(Request::Ids(ids));
        }

        let mut kinds: Vec<Kind> = Vec::new();
        let mut counts: Vec<&str> = Vec::new();
        let mut filter = Filter::default();
        let mut text: Vec<&str> = Vec::new();
        for &w in &words {
            let lower = w.to_ascii_lowercase();
            if let Some(k) = Kind::parse(&lower) {
                if !kinds.contains(&k) {
                    kinds.push(k);
                }
            } else if looks_numeric(w) {
                counts.push(w);
            } else if !filter.add(&lower, labels) {
                text.push(w);
            }
        }

        // Anything that isn't a keyword, count or filter makes it a search,
        // numbers included (`/solve 3sum`); a keyword makes it a mistake.
        if let Some(word) = text.first() {
            return match kinds.first() {
                Some(k) => Err(ParseError::NotAFilter {
                    word: word.to_string(),
                    kind: k.name(),
                }),
                None => Ok(Request::Search {
                    text: words.join(" "),
                    count: n.unwrap_or(1),
                }),
            };
        }
        if kinds.len() > 1 {
            return Err(ParseError::TwoKinds);
        }
        let count = match counts.as_slice() {
            [] => n.unwrap_or(1),
            [w] => match w.parse::<usize>() {
                Ok(c) if c > 0 && n.is_none_or(|n| n == c) => c,
                Ok(c) if c > 0 => return Err(ParseError::TwoCounts),
                _ => return Err(ParseError::BadCount(w.to_string())),
            },
            _ => return Err(ParseError::TwoCounts),
        };
        Ok(Request::Pick {
            kind: kinds.first().copied().unwrap_or(Kind::Need),
            filter,
            count,
        })
    }
}

/// A number-like word (`12`, `-1`, `1.5`): meant as an id or count, so
/// never used as search words.
pub fn looks_numeric(word: &str) -> bool {
    word.chars().any(|c| c.is_ascii_digit())
        && word
            .chars()
            .all(|c| c.is_ascii_digit() || matches!(c, '-' | '+' | '.'))
}

/// Anything with a difficulty, tags and companies.
pub trait Labeled {
    fn difficulty(&self) -> Difficulty;
    fn tags(&self) -> &[String];
    fn companies(&self) -> &[String];
}

impl Labeled for Question {
    fn difficulty(&self) -> Difficulty {
        self.meta.difficulty
    }
    fn tags(&self) -> &[String] {
        &self.meta.tags
    }
    fn companies(&self) -> &[String] {
        &self.meta.companies
    }
}

impl Labeled for QuestionStat {
    fn difficulty(&self) -> Difficulty {
        self.difficulty
    }
    fn tags(&self) -> &[String] {
        &self.tags
    }
    fn companies(&self) -> &[String] {
        &self.companies
    }
}

/// Tag, company and difficulty words: `google dfs easy`. Kept sorted, so
/// the order they were typed in doesn't matter.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Filter {
    labels: Vec<String>,
    levels: Vec<Difficulty>,
}

impl Filter {
    /// Adds `word` (lowercase) if it names a difficulty or one of `labels`.
    pub fn add(&mut self, word: &str, labels: &[String]) -> bool {
        if let Some(d) = difficulty(word) {
            if let Err(i) = self.levels.binary_search(&d) {
                self.levels.insert(i, d);
            }
            true
        } else if labels.iter().any(|l| l == word) {
            if let Err(i) = self.labels.binary_search_by(|l| l.as_str().cmp(word)) {
                self.labels.insert(i, word.to_string());
            }
            true
        } else {
            false
        }
    }

    pub fn is_empty(&self) -> bool {
        self.labels.is_empty() && self.levels.is_empty()
    }

    /// Every label, and any of the difficulties.
    pub fn matches(&self, q: &impl Labeled) -> bool {
        (self.levels.is_empty() || self.levels.contains(&q.difficulty()))
            && self
                .labels
                .iter()
                .all(|l| q.tags().contains(l) || q.companies().contains(l))
    }

    /// `google dfs easy`.
    pub fn words(&self) -> String {
        let levels = self.levels.iter().map(|d| d.to_string());
        self.labels
            .iter()
            .cloned()
            .chain(levels)
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn difficulty(word: &str) -> Option<Difficulty> {
    match word {
        "easy" => Some(Difficulty::Easy),
        "medium" | "med" => Some(Difficulty::Medium),
        "hard" => Some(Difficulty::Hard),
        _ => None,
    }
}

/// Untried questions come mediums first (asked most), then easies, then
/// hards.
pub fn untried_rank(d: Difficulty) -> u8 {
    match d {
        Difficulty::Medium => 0,
        Difficulty::Easy => 1,
        Difficulty::Hard => 2,
    }
}

/// What was chosen, and how.
#[derive(Debug)]
pub struct Selection {
    pub ids: Vec<u32>,
    /// `need`, `random` or `search`.
    pub mode: &'static str,
    /// The filter or search words, if any.
    pub query: Option<String>,
    /// Why each was picked (`need` only).
    pub reasons: Vec<Suggestion>,
    /// How many were asked for.
    pub wanted: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum SelectError {
    #[error("no question matches `{0}`")]
    NoMatch(String),
    #[error("could not read your history: {0:#}")]
    History(anyhow::Error),
}

/// Runs a search or pick request. Ids are the caller's to check.
pub fn select(
    request: &Request,
    library: &impl Library,
    dice: &impl Dice,
    now: Timestamp,
) -> Result<Selection, SelectError> {
    match request {
        Request::Ids(ids) => Ok(Selection {
            ids: ids.clone(),
            mode: "id",
            query: None,
            reasons: Vec::new(),
            wanted: ids.len(),
        }),
        Request::Search { text, count } => {
            let ids: Vec<u32> = library.search(text).into_iter().take(*count).collect();
            if ids.is_empty() {
                return Err(SelectError::NoMatch(text.clone()));
            }
            Ok(Selection {
                ids,
                mode: "search",
                query: Some(text.clone()),
                reasons: Vec::new(),
                wanted: *count,
            })
        }
        Request::Pick {
            kind,
            filter,
            count,
        } => {
            let (questions, topics) = library.history().map_err(SelectError::History)?;
            let pool: Vec<QuestionStat> = questions
                .into_iter()
                .filter(|q| filter.matches(q))
                .collect();
            if pool.is_empty() {
                return Err(SelectError::NoMatch(filter.words()));
            }
            let picks = match kind {
                Kind::Need => Need::new(now).pick(&pool, &topics, *count),
                Kind::Random => Random::new(dice).pick(&pool, &topics, *count),
            };
            Ok(Selection {
                ids: picks.iter().map(|p| p.question_id).collect(),
                mode: kind.name(),
                query: (!filter.is_empty()).then(|| filter.words()),
                reasons: if *kind == Kind::Need {
                    picks
                } else {
                    Vec::new()
                },
                wanted: *count,
            })
        }
    }
}

#[cfg(test)]
pub(crate) mod mock;

#[cfg(test)]
mod tests;
