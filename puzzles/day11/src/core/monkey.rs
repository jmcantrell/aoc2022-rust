use std::collections::VecDeque;
use std::convert::TryFrom;

use anyhow::Context;

use crate::core::parse::ensure_prefix;

use super::{Operation, Test};

pub type Item = usize;

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items: VecDeque<Item>,
    pub operation: Operation,
    pub test: Test,
    pub inspections: usize,
}

impl Monkey {
    pub fn throw_item(&mut self) -> Option<Item> {
        let item = self.items.pop_front()?;
        self.inspections += 1;
        Some(item)
    }
}

impl TryFrom<&str> for Monkey {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        fn parse_item(s: &str) -> anyhow::Result<Item> {
            s.trim()
                .parse()
                .with_context(|| format!("invalid item: {s:?}"))
        }

        fn parse_items(s: &str) -> anyhow::Result<VecDeque<Item>> {
            ensure_prefix(s, "Starting items:")?
                .split(',')
                .enumerate()
                .map(|(i, s)| parse_item(s).with_context(|| format!("item number {}", i + 1)))
                .collect::<Result<VecDeque<_>, _>>()
                .with_context(|| format!("invalid starting items: {s:?}"))
        }

        fn parse_test<'a>(iter: &mut impl Iterator<Item = &'a str>) -> anyhow::Result<Test> {
            let s = iter.take(3).collect::<Vec<_>>().join("\n");
            Test::try_from(s.as_str()).with_context(|| format!("invalid test: {s:?}"))
        }

        let mut lines = s.lines().skip(1);

        let items = parse_items(lines.next().context("missing starting items")?)?;
        let operation = lines.next().context("missing operation")?.try_into()?;
        let test = parse_test(&mut lines)?;

        Ok(Self {
            items,
            operation,
            test,
            inspections: 0,
        })
    }
}

#[derive(Debug, Clone)]
pub struct MonkeyTroop {
    pub monkeys: Vec<Monkey>,
}

impl MonkeyTroop {
    fn throw_items<F>(&mut self, from: usize, worry_reducer: &F)
    where
        F: Fn(Item) -> Item,
    {
        while let Some(item) = self.monkeys[from].throw_item() {
            let item = worry_reducer(self.monkeys[from].operation.eval(item));
            let to = self.monkeys[from].test.eval(item);
            self.monkeys[to].items.push_back(item);
        }
    }

    pub fn iterate<F>(&mut self, worry_reducer: &F)
    where
        F: Fn(Item) -> Item,
    {
        for i in 0..self.monkeys.len() {
            self.throw_items(i, worry_reducer);
        }
    }
}

impl TryFrom<&str> for MonkeyTroop {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        fn parse_monkey(s: &str) -> anyhow::Result<Monkey> {
            s.try_into()
                .with_context(|| format!("invalid monkey: {s:?}"))
        }

        let monkeys = s
            .split("\n\n")
            .enumerate()
            .map(|(i, s)| parse_monkey(s).with_context(|| format!("monkey number {}", i + 1)))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { monkeys })
    }
}
