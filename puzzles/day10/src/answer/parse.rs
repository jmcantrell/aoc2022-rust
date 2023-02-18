use anyhow::Context;

use aoc::Input;

use crate::core::{Command, Program};

pub type Parsed = Program;

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
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
