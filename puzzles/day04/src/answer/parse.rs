use anyhow::Context;

use aoc::Input;

use crate::core::{AssignmentPair, AssignmentPairs};

type Parsed = AssignmentPairs;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
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

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    parse(input)
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    parse(input)
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
