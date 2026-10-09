//! `random`: any of them, by chance.

use std::cell::Cell;

use super::{Dice, Strategy};
use crate::model::{LabelStat, QuestionStat, Suggestion};

pub struct Random<'d, D: Dice> {
    dice: &'d D,
}

impl<'d, D: Dice> Random<'d, D> {
    pub fn new(dice: &'d D) -> Self {
        Random { dice }
    }
}

impl<D: Dice> Strategy for Random<'_, D> {
    fn pick(&self, pool: &[QuestionStat], _: &[LabelStat], count: usize) -> Vec<Suggestion> {
        // Partial Fisher-Yates: the first `count` slots get a fair draw.
        let mut order: Vec<&QuestionStat> = pool.iter().collect();
        let take = count.min(order.len());
        for i in 0..take {
            let j = i + self.dice.below(order.len() - i);
            order.swap(i, j);
        }
        order
            .into_iter()
            .take(take)
            .map(|q| {
                Suggestion::builder()
                    .question_id(q.id)
                    .title(q.title.clone())
                    .reason("random".into())
                    .build()
            })
            .collect()
    }
}

/// xorshift seeded from the clock; good enough for picking practice.
pub struct Clock(Cell<u64>);

impl Clock {
    pub fn new() -> Clock {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0x9e37_79b9, |d| d.as_nanos() as u64)
            | 1;
        Clock(Cell::new(seed))
    }
}

impl Dice for Clock {
    fn below(&self, n: usize) -> usize {
        let mut x = self.0.get();
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0.set(x);
        (x % n as u64) as usize
    }
}
