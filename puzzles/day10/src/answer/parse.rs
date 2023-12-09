use anyhow::Context;

use aoc::Input;

use crate::core::{Command, Program};

type Parsed = Program;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    fn parse_command(s: &str) -> anyhow::Result<Command> {
        s.try_into()
            .with_context(|| format!("invalid command: {s:?}"))
    }

    input
        .lines()
        .enumerate()
        .map(|(i, s)| parse_command(s).with_context(|| format!("line number {}", i + 1)))
        .collect::<Result<Vec<_>, _>>()
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

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        dbg!(parse(INPUT)?);
        Ok(())
    }
}
