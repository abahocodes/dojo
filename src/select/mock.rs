//! Stand-ins for the bank, your history and chance.

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;

use jiff::{Timestamp, ToSpan};

use super::{Dice, Library};
use crate::model::{LabelStat, QuestionStat};
use crate::questions::{Companies, Difficulty, Frequency};

pub fn now() -> Timestamp {
    "2026-10-09T12:00:00Z".parse().unwrap()
}

pub fn hours_ago(h: i64) -> Timestamp {
    now().checked_sub(h.hours()).unwrap()
}

/// An untried question. Known company names in `labels` become companies
/// asking it regularly (3); the rest are topics.
pub fn question(id: u32, difficulty: Difficulty, labels: &[&str]) -> QuestionStat {
    let (companies, tags): (Vec<&str>, Vec<&str>) =
        labels.iter().partition(|l| COMPANIES.contains(l));
    QuestionStat::builder()
        .id(id)
        .title(format!("Q{id}"))
        .difficulty(difficulty)
        .tags(tags.into_iter().map(String::from).collect())
        .companies(Companies(
            companies
                .into_iter()
                .map(|c| (c.to_string(), Frequency(3)))
                .collect(),
        ))
        .attempts(0)
        .solved(false)
        .target_secs(900)
        .build()
}

const COMPANIES: &[&str] = &["google", "meta", "amazon"];

/// `company` asks `q` this often.
pub fn asked(mut q: QuestionStat, company: &str, frequency: u8) -> QuestionStat {
    q.companies.0.insert(company.into(), Frequency(frequency));
    q
}

/// Passed with `score`, last tried `hours` ago.
pub fn solved(mut q: QuestionStat, score: f64, hours: i64) -> QuestionStat {
    q.attempts += 1;
    q.solved = true;
    q.score = Some(score);
    q.last_attempt = Some(hours_ago(hours));
    q
}

/// Tried without passing, `hours` ago.
pub fn tried(mut q: QuestionStat, hours: i64) -> QuestionStat {
    q.attempts += 1;
    q.score = Some(0.0);
    q.last_attempt = Some(hours_ago(hours));
    q
}

pub fn topic(label: &str, mastery: f64, attempted: usize) -> LabelStat {
    LabelStat::builder()
        .label(label.into())
        .mastery(mastery)
        .attempted(attempted)
        .total(attempted.max(1))
        .confidence(0)
        .build()
}

/// A library that hands back what it was given and counts what it's asked.
#[derive(Default)]
pub struct MockLibrary {
    pub questions: Vec<QuestionStat>,
    pub topics: Vec<LabelStat>,
    /// What `search` returns, whatever the text.
    pub matches: Vec<u32>,
    pub broken: bool,
    pub history_calls: Cell<usize>,
    pub searched: RefCell<Vec<String>>,
}

impl MockLibrary {
    pub fn with(questions: Vec<QuestionStat>) -> MockLibrary {
        MockLibrary {
            questions,
            ..MockLibrary::default()
        }
    }
}

impl Library for MockLibrary {
    fn labels(&self) -> Vec<String> {
        let mut out: Vec<String> = self
            .questions
            .iter()
            .flat_map(|q| q.tags.iter().chain(q.companies.names()).cloned())
            .collect();
        out.sort();
        out.dedup();
        out
    }

    fn history(&self) -> anyhow::Result<(Vec<QuestionStat>, Vec<LabelStat>)> {
        self.history_calls.set(self.history_calls.get() + 1);
        if self.broken {
            anyhow::bail!("disk on fire");
        }
        Ok((self.questions.clone(), self.topics.clone()))
    }

    fn search(&self, text: &str) -> Vec<u32> {
        self.searched.borrow_mut().push(text.to_string());
        self.matches.clone()
    }
}

/// Dice that roll a script (each roll taken modulo `n`), then zeros, and
/// remember every `n` they were asked for.
#[derive(Default)]
pub struct MockDice {
    rolls: RefCell<VecDeque<usize>>,
    pub asked: RefCell<Vec<usize>>,
}

impl MockDice {
    pub fn rolling(rolls: &[usize]) -> MockDice {
        MockDice {
            rolls: RefCell::new(rolls.iter().copied().collect()),
            asked: RefCell::default(),
        }
    }
}

impl Dice for MockDice {
    fn below(&self, n: usize) -> usize {
        self.asked.borrow_mut().push(n);
        self.rolls.borrow_mut().pop_front().unwrap_or(0) % n
    }
}
