//! Choosing what to practice. `/solve` words parse into a [`Request`]
//! regardless of order (`google need 3` is `need google 3`), and a
//! [`Strategy`] picks from the questions a [`Filter`] lets through.
//! Questions and history come from a [`Library`] and chance from [`Dice`],
//! so selection is pure and testable with mocks.

mod need;
mod random;

use std::collections::{BTreeMap, BTreeSet};

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::model::{LabelStat, QuestionStat, Suggestion};
use crate::questions::{Companies, Difficulty, Question};

pub use need::Need;
pub use random::{Clock, Random};

/// Where questions and your history come from.
pub trait Library {
    /// The tags, companies and title words `/solve` understands.
    fn vocabulary(&self) -> Vocabulary;
    /// The questions you can practice now (in your language) with your
    /// history on each, and your mastery per topic.
    fn history(&self) -> anyhow::Result<(Vec<QuestionStat>, Vec<LabelStat>)>;
    /// The question open now, which a new pick replaces.
    fn open(&self) -> Option<u32>;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    /// Where you need practice.
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

/// How many questions `/solve` offers when no count is given.
pub const OFFER: usize = 5;

/// What `/solve` asked for.
#[derive(Debug, PartialEq)]
pub enum Request {
    /// `/solve 3 7`, `/solve two-sum`: these questions, in this order.
    Ids(Vec<u32>),
    /// `/solve need google 3`, `/solve dfs easy`, `/solve random`: up to
    /// `count` questions to choose from.
    Pick {
        kind: Kind,
        filter: Filter,
        count: usize,
    },
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseError {
    #[error(
        "what should we practice?  ·  /solve 12, /solve need, /solve google dfs  ·  /help solve"
    )]
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
    #[error("`{word}` isn't a tag, company or difficulty{hint}  ·  /list {word} searches titles")]
    Unknown { word: String, hint: String },
    #[error("`{0}` names a question; give questions alone (/solve {0}) or use filters")]
    QuestionAndFilters(String),
    #[error("`{0}` isn't an option  ·  /help solve")]
    UnknownOption(String),
    #[error("{0} needs a value  ·  Tab lists them")]
    NoValue(String),
    #[error("`{value}` isn't a {what}{hint}")]
    NotA {
        what: &'static str,
        value: String,
        hint: String,
    },
}

/// What an option word asks for.
enum Opt {
    Count,
    Kind(Kind),
    Tag,
    Company,
    Difficulty,
}

impl Opt {
    fn parse(flag: &str) -> Option<Opt> {
        Some(match flag {
            "-n" | "--count" => Opt::Count,
            "--need" => Opt::Kind(Kind::Need),
            "--random" => Opt::Kind(Kind::Random),
            "-t" | "--tag" => Opt::Tag,
            "-c" | "--company" => Opt::Company,
            "-d" | "--difficulty" => Opt::Difficulty,
            _ => return None,
        })
    }

    fn takes_value(&self) -> bool {
        !matches!(self, Opt::Kind(_))
    }
}

/// The `/solve` options, for help and completion: (flags, value, about).
pub const OPTIONS: &[(&str, &str, &str)] = &[
    ("--need", "", "where you need practice"),
    ("--random", "", "any of them, by chance"),
    (
        "-c, --company",
        "<name>",
        "only questions this company asks",
    ),
    ("-t, --tag", "<name>", "only questions with this tag"),
    ("-d, --difficulty", "<level>", "easy, medium or hard"),
    ("-n, --count", "<N>", "how many to choose from (default 5)"),
];

/// Whether `word` is an option rather than a value (`-1` is a value).
fn is_option(word: &str) -> bool {
    word.starts_with('-') && word.len() > 1 && !looks_numeric(word)
}

/// `  ·  did you mean a, b?`, or nothing.
fn did_you_mean(close: &[&str]) -> String {
    if close.is_empty() {
        String::new()
    } else {
        format!("  ·  did you mean {}?", close.join(", "))
    }
}

impl Request {
    /// `/solve` arguments, given the words the bank knows. Filter words or
    /// a count alone use `default`.
    ///
    /// Bare words mean, in order: need or random, an id or count, a
    /// question's slug, a difficulty, a tag, a company. Anything else is a
    /// mistake, never a guess. Options say which one outright
    /// (`--company medium`).
    pub fn parse(args: &[&str], vocab: &Vocabulary, default: Kind) -> Result<Request, ParseError> {
        let mut n: Option<usize> = None;
        let mut kinds: Vec<Kind> = Vec::new();
        let mut filter = Filter::default();
        let mut words: Vec<&str> = Vec::new();
        // An option other than the count makes this a pick.
        let mut chose = false;

        let mut it = args.iter().copied().peekable();
        while let Some(a) = it.next() {
            if !is_option(a) {
                words.push(a);
                continue;
            }
            let (flag, inline) = match a.split_once('=') {
                Some((f, v)) => (f, Some(v)),
                None => (a, None),
            };
            let opt = Opt::parse(flag).ok_or_else(|| ParseError::UnknownOption(a.to_string()))?;
            let value = if opt.takes_value() {
                match inline.or_else(|| it.next_if(|v| !is_option(v))) {
                    Some(v) if !v.is_empty() => Some(v),
                    _ if matches!(opt, Opt::Count) => return Err(ParseError::BadN),
                    _ => return Err(ParseError::NoValue(flag.to_string())),
                }
            } else if inline.is_some() {
                return Err(ParseError::UnknownOption(a.to_string()));
            } else {
                None
            };
            match opt {
                Opt::Count => match value.and_then(|v| v.parse().ok()) {
                    Some(v) if v > 0 => n = Some(v),
                    _ => return Err(ParseError::BadN),
                },
                Opt::Kind(k) => {
                    chose = true;
                    if !kinds.contains(&k) {
                        kinds.push(k);
                    }
                }
                Opt::Tag | Opt::Company | Opt::Difficulty => {
                    chose = true;
                    for v in value
                        .unwrap_or_default()
                        .split(',')
                        .filter(|v| !v.is_empty())
                    {
                        filter.named(&opt, &v.to_ascii_lowercase(), vocab)?;
                    }
                }
            }
        }

        if words.is_empty() && !chose && n.is_none() {
            return Err(ParseError::Empty);
        }

        // Only numbers and slugs: those questions (`/solve 3 3` practices
        // #3 once).
        let slug = |w: &str| vocab.slugs.get(&w.to_ascii_lowercase()).copied();
        if !chose
            && !words.is_empty()
            && words.iter().all(|w| looks_numeric(w) || slug(w).is_some())
        {
            let mut ids: Vec<u32> = Vec::new();
            for w in &words {
                let id = match slug(w) {
                    Some(id) => id,
                    None => w.parse().map_err(|_| ParseError::BadId(w.to_string()))?,
                };
                if !ids.contains(&id) {
                    ids.push(id);
                }
            }
            return Ok(Request::Ids(ids));
        }

        let mut counts: Vec<&str> = Vec::new();
        for &w in &words {
            let lower = w.to_ascii_lowercase();
            if let Some(k) = Kind::parse(&lower) {
                if !kinds.contains(&k) {
                    kinds.push(k);
                }
            } else if looks_numeric(w) {
                counts.push(w);
            } else if filter.add(&lower, vocab) {
            } else if slug(w).is_some() {
                return Err(ParseError::QuestionAndFilters(w.to_string()));
            } else {
                let close = vocab.close(&lower);
                return Err(ParseError::Unknown {
                    word: w.to_string(),
                    hint: did_you_mean(&close.iter().map(String::as_str).collect::<Vec<_>>()),
                });
            }
        }
        if kinds.len() > 1 {
            return Err(ParseError::TwoKinds);
        }
        let count = match counts.as_slice() {
            [] => n.unwrap_or(OFFER),
            [w] => match w.parse::<usize>() {
                Ok(c) if c > 0 && n.is_none_or(|n| n == c) => c,
                Ok(c) if c > 0 => return Err(ParseError::TwoCounts),
                _ => return Err(ParseError::BadCount(w.to_string())),
            },
            _ => return Err(ParseError::TwoCounts),
        };
        Ok(Request::Pick {
            kind: kinds.first().copied().unwrap_or(default),
            filter,
            count,
        })
    }
}

/// The words `/solve` knows: tags, companies and question slugs.
#[derive(Debug, Default, Clone)]
pub struct Vocabulary {
    pub tags: BTreeSet<String>,
    pub companies: BTreeSet<String>,
    pub slugs: BTreeMap<String, u32>,
}

impl Vocabulary {
    pub fn of<'a, L: Labeled + 'a>(questions: impl IntoIterator<Item = &'a L>) -> Vocabulary {
        let mut v = Vocabulary::default();
        for q in questions {
            v.tags.extend(q.tags().iter().cloned());
            v.companies.extend(q.companies().names().cloned());
            if let Some(slug) = q.slug() {
                v.slugs.insert(slug.to_string(), q.id());
            }
        }
        v
    }

    /// Every word a filter or strategy can be.
    fn names(&self) -> Vec<String> {
        let mut names: Vec<String> = ["need", "random", "easy", "medium", "hard"]
            .into_iter()
            .map(String::from)
            .chain(self.tags.iter().cloned())
            .chain(self.companies.iter().cloned())
            .collect();
        let mut seen = BTreeSet::new();
        names.retain(|n| seen.insert(n.clone()));
        names
    }

    /// Up to three known words `word` was likely meant as: ones it starts,
    /// else the closest fuzzy matches.
    pub fn close(&self, word: &str) -> Vec<String> {
        let names = self.names();
        let started: Vec<String> = names
            .iter()
            .filter(|n| n.starts_with(word))
            .take(3)
            .cloned()
            .collect();
        if !started.is_empty() {
            return started;
        }
        crate::questions::search::rank(word, &names, |n| n.clone())
            .into_iter()
            .take(3)
            .cloned()
            .collect()
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
    fn id(&self) -> u32;
    /// The question's slug, when known.
    fn slug(&self) -> Option<&str>;
    fn difficulty(&self) -> Difficulty;
    fn tags(&self) -> &[String];
    fn companies(&self) -> &Companies;
}

impl Labeled for Question {
    fn id(&self) -> u32 {
        self.meta.id
    }
    fn slug(&self) -> Option<&str> {
        Some(&self.meta.slug)
    }
    fn difficulty(&self) -> Difficulty {
        self.meta.difficulty
    }
    fn tags(&self) -> &[String] {
        &self.meta.tags
    }
    fn companies(&self) -> &Companies {
        &self.meta.companies
    }
}

impl Labeled for QuestionStat {
    fn id(&self) -> u32 {
        self.id
    }
    fn slug(&self) -> Option<&str> {
        None
    }
    fn difficulty(&self) -> Difficulty {
        self.difficulty
    }
    fn tags(&self) -> &[String] {
        &self.tags
    }
    fn companies(&self) -> &Companies {
        &self.companies
    }
}

/// Tag, company and difficulty words: `google dfs easy`. Kept sorted, so
/// the order they were typed in doesn't matter.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Filter {
    tags: Vec<String>,
    companies: Vec<String>,
    levels: Vec<Difficulty>,
}

fn insert_sorted<T: Ord>(list: &mut Vec<T>, item: T) {
    if let Err(i) = list.binary_search(&item) {
        list.insert(i, item);
    }
}

impl Filter {
    /// Adds a bare word (lowercase) if it names a difficulty, tag or
    /// company, first match wins.
    pub fn add(&mut self, word: &str, vocab: &Vocabulary) -> bool {
        if let Some(d) = difficulty(word) {
            insert_sorted(&mut self.levels, d);
        } else if vocab.tags.contains(word) {
            insert_sorted(&mut self.tags, word.to_string());
        } else if vocab.companies.contains(word) {
            insert_sorted(&mut self.companies, word.to_string());
        } else {
            return false;
        }
        true
    }

    /// Adds the value of `--tag`, `--company` or `--difficulty`.
    fn named(&mut self, opt: &Opt, value: &str, vocab: &Vocabulary) -> Result<(), ParseError> {
        let levels = ["easy", "medium", "hard"].map(String::from);
        let (what, known): (&'static str, Vec<&String>) = match opt {
            Opt::Tag => ("tag", vocab.tags.iter().collect()),
            Opt::Company => ("company", vocab.companies.iter().collect()),
            _ => ("difficulty", levels.iter().collect()),
        };
        if !known.iter().any(|k| *k == value) {
            let close: Vec<&str> = known
                .iter()
                .filter(|k| k.starts_with(value))
                .chain(
                    known
                        .iter()
                        .filter(|k| !k.starts_with(value) && k.contains(value)),
                )
                .take(3)
                .map(|k| k.as_str())
                .collect();
            return Err(ParseError::NotA {
                what,
                value: value.to_string(),
                hint: if close.is_empty() {
                    "  ·  Tab lists them".into()
                } else {
                    format!("  ·  did you mean {}?", close.join(", "))
                },
            });
        }
        match opt {
            Opt::Tag => insert_sorted(&mut self.tags, value.to_string()),
            Opt::Company => insert_sorted(&mut self.companies, value.to_string()),
            _ => insert_sorted(&mut self.levels, difficulty(value).expect("checked")),
        }
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.tags.is_empty() && self.companies.is_empty() && self.levels.is_empty()
    }

    /// The companies asked about.
    pub fn companies(&self) -> &[String] {
        &self.companies
    }

    /// Every tag and company, and any of the difficulties.
    pub fn matches(&self, q: &impl Labeled) -> bool {
        (self.levels.is_empty() || self.levels.contains(&q.difficulty()))
            && self.tags.iter().all(|t| q.tags().contains(t))
            && self.companies.iter().all(|c| q.companies().contains(c))
    }

    /// `google dfs easy`.
    pub fn words(&self) -> String {
        let mut labels: Vec<&String> = self.tags.iter().chain(&self.companies).collect();
        labels.sort();
        let levels = self.levels.iter().map(|d| d.to_string());
        labels
            .into_iter()
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

/// How `/solve` picks: `[solve]` in the config file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Preferences {
    /// What `/solve` does with only tags or a count (`/solve google`):
    /// `need` or `random`.
    pub strategy: Kind,
}

impl Default for Preferences {
    fn default() -> Self {
        Preferences {
            strategy: Kind::Need,
        }
    }
}

/// What was chosen, and how.
#[derive(Debug)]
pub struct Selection {
    pub ids: Vec<u32>,
    /// `id`, `need` or `random`.
    pub mode: &'static str,
    /// The filter words, if any.
    pub query: Option<String>,
    /// Why each was picked (`need` and `random`).
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

/// Runs a pick request. Ids are the caller's to check.
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
        Request::Pick {
            kind,
            filter,
            count,
        } => {
            let (questions, topics) = library.history().map_err(SelectError::History)?;
            let mut pool: Vec<QuestionStat> = questions
                .into_iter()
                .filter(|q| filter.matches(q))
                .collect();
            if pool.is_empty() {
                return Err(SelectError::NoMatch(filter.words()));
            }
            // Asking for something new while a question is open moves on
            // from it, so it isn't picked again (unless it's all there is).
            // Asking for it by id or name still restarts it.
            if pool.len() > 1
                && let Some(open) = library.open()
            {
                pool.retain(|q| q.id != open);
            }
            let picks = match kind {
                Kind::Need => Need::new(now)
                    .focus(filter.companies())
                    .pick(&pool, &topics, *count),
                Kind::Random => Random::new(dice).pick(&pool, &topics, *count),
            };
            Ok(Selection {
                ids: picks.iter().map(|p| p.question_id).collect(),
                mode: kind.name(),
                query: (!filter.is_empty()).then(|| filter.words()),
                reasons: picks,
                wanted: *count,
            })
        }
    }
}

#[cfg(test)]
pub(crate) mod mock;

#[cfg(test)]
mod tests;
