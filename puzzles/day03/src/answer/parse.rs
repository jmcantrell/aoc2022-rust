use anyhow::{anyhow, Context};

use aoc::Input;

use crate::core::{Priority, Rucksack, Rucksacks};

type Parsed = Rucksacks;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    fn parse_priority(c: char) -> anyhow::Result<Priority> {
        match c {
            'a'..='z' => Ok('a' as usize - 1),
            'A'..='Z' => Ok('A' as usize - 26 - 1),
            _ => Err(anyhow!("invalid priority: {:?}", c)),
        }
        .map(|offset| c as usize - offset)
    }

    fn parse_rucksack(s: &str) -> anyhow::Result<Rucksack> {
        s.chars()
            .enumerate()
            .map(|(i, c)| parse_priority(c).with_context(|| format!("column number {}", i + 1)))
            .collect()
    }

    fn parse_rucksacks(s: &str) -> anyhow::Result<Rucksacks> {
        s.lines()
            .enumerate()
            .map(|(i, s)| parse_rucksack(s).with_context(|| format!("line number {}", i + 1)))
            .collect()
    }

    parse_rucksacks(input).context("unable to parse rucksacks")
}

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    parse(input)
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    parse(input)
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use super::*;

    const INPUT: Input = include_str!("../../input-test");

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        dbg!(parse(INPUT)?);
        Ok(())
    }
}
