use anyhow::Context;

use aoc::Input;

use crate::core::{AssignmentPair, AssignmentPairs};

pub type Parsed = AssignmentPairs;

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
    fn parse_assignment_pair(s: &str) -> anyhow::Result<AssignmentPair> {
        s.try_into().context("unable to parse assignment pair")
    }

    fn parse_assignment_pairs(s: &str) -> anyhow::Result<AssignmentPairs> {
        s.lines()
            .enumerate()
            .map(|(i, s)| {
                parse_assignment_pair(s).with_context(|| format!("line number {}", i + 1))
            })
            .collect()
    }

    parse_assignment_pairs(input).context("unable to parse assignment pairs")
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn parse() -> anyhow::Result<()> {
        dbg!(super::parse(INPUT)?);
        Ok(())
    }
}
