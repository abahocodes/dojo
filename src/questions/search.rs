//! Question search. Words that exactly name a tag, company or difficulty act
//! as filters; anything else is fuzzy-matched against title, slug and labels.

use nucleo_matcher::pattern::{CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher, Utf32Str};

use super::{Bank, Question};
use crate::select::Filter;

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

pub fn search<'a>(bank: &'a Bank, query: &str) -> Vec<&'a Question> {
    let labels = bank.labels();
    let mut filter = Filter::default();
    let rest: Vec<&str> = query
        .split_whitespace()
        .filter(|w| !filter.add(&w.to_ascii_lowercase(), &labels))
        .collect();
    let candidates: Vec<&Question> = bank.all().iter().filter(|q| filter.matches(*q)).collect();

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
        q.meta
            .companies
            .names()
            .cloned()
            .collect::<Vec<_>>()
            .join(" ")
    )
}
