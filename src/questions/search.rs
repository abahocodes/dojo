//! Question search. Words that exactly name a tag, company or difficulty act
//! as filters; anything else is fuzzy-matched against title, slug and labels.

use nucleo_matcher::pattern::{CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher, Utf32Str};

use super::{Bank, Difficulty, Question};

/// Fuzzy-ranks `items` against `query`, best first. Empty query keeps order.
pub fn rank<'a, T>(query: &str, items: &'a [T], key: impl Fn(&T) -> String) -> Vec<&'a T> {
    if query.trim().is_empty() {
        return items.iter().collect();
    }
    let mut matcher = Matcher::new(Config::DEFAULT);
    let pattern = Pattern::parse(query, CaseMatching::Ignore, Normalization::Smart);
    let mut buf = Vec::new();
    let mut scored: Vec<(u32, usize, &T)> = items
        .iter()
        .enumerate()
        .filter_map(|(i, item)| {
            let hay = key(item);
            pattern
                .score(Utf32Str::new(&hay, &mut buf), &mut matcher)
                .map(|s| (s, i, item))
        })
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    scored.into_iter().map(|(_, _, item)| item).collect()
}

fn difficulty(word: &str) -> Option<Difficulty> {
    match word {
        "easy" => Some(Difficulty::Easy),
        "medium" | "med" => Some(Difficulty::Medium),
        "hard" => Some(Difficulty::Hard),
        _ => None,
    }
}

/// Words that name a tag, company or difficulty: `google dfs easy`.
#[derive(Debug, Default)]
pub struct Filter {
    labels: Vec<String>,
    levels: Vec<Difficulty>,
}

impl Filter {
    /// Every word must be a label or difficulty; `Err` names the first one
    /// that isn't.
    pub fn parse<'w>(bank: &Bank, words: &[&'w str]) -> Result<Filter, &'w str> {
        match Filter::split(bank, words) {
            (filter, rest) if rest.is_empty() => Ok(filter),
            (_, rest) => Err(rest[0]),
        }
    }

    /// The filter words, and the rest in order.
    fn split<'w>(bank: &Bank, words: &[&'w str]) -> (Filter, Vec<&'w str>) {
        let labels = bank.labels();
        let mut filter = Filter::default();
        let mut rest = Vec::new();
        for &word in words {
            let lower = word.to_ascii_lowercase();
            if let Some(d) = difficulty(&lower) {
                filter.levels.push(d);
            } else if labels.contains(&lower) {
                filter.labels.push(lower);
            } else {
                rest.push(word);
            }
        }
        (filter, rest)
    }

    /// Any of the difficulties, and every label.
    pub fn matches(&self, q: &Question) -> bool {
        (self.levels.is_empty() || self.levels.contains(&q.meta.difficulty))
            && self
                .labels
                .iter()
                .all(|f| q.meta.tags.contains(f) || q.meta.companies.contains(f))
    }
}

pub fn search<'a>(bank: &'a Bank, query: &str) -> Vec<&'a Question> {
    let words: Vec<&str> = query.split_whitespace().collect();
    let (filter, rest) = Filter::split(bank, &words);
    let candidates: Vec<&Question> = bank.all().iter().filter(|q| filter.matches(q)).collect();

    rank(&rest.join(" "), &candidates, |q| haystack(q))
        .into_iter()
        .copied()
        .collect()
}

fn haystack(q: &Question) -> String {
    format!(
        "{} {} {} {}",
        q.meta.title,
        q.meta.slug,
        q.meta.tags.join(" "),
        q.meta.companies.join(" ")
    )
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::company(&["google"], Ok(()))]
    #[case::mixed_case(&["Google", "DFS", "Easy"], Ok(()))]
    #[case::none(&[], Ok(()))]
    #[case::free_text(&["google", "two"], Err("two"))]
    fn parses_filter_words(#[case] words: &[&str], #[case] expected: Result<(), &str>) {
        let bank = Bank::embedded();
        assert_eq!(Filter::parse(&bank, words).map(|_| ()), expected);
    }

    #[test]
    fn filter_needs_every_label_and_any_difficulty() {
        let bank = Bank::embedded();
        let f = Filter::parse(&bank, &["google", "dfs", "easy", "medium"]).unwrap();
        let hits: Vec<&Question> = bank.all().iter().filter(|q| f.matches(q)).collect();
        assert!(!hits.is_empty());
        for q in hits {
            assert!(q.meta.companies.iter().any(|c| c == "google"));
            assert!(q.meta.tags.iter().any(|t| t == "dfs"));
            assert_ne!(q.meta.difficulty, Difficulty::Hard);
        }
    }
}
