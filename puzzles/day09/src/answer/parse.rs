use anyhow::Context;

use aoc::Input;

use crate::core::Movement;

type Parsed = Vec<Movement>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    input
        .lines()
        .enumerate()
        .map(|(i, s)| {
            s.try_into()
                .with_context(|| format!("line number {}", i + 1))
        })
        .collect()
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

    const INPUT1: Input = include_str!("../../input-test-1");
    const INPUT2: Input = include_str!("../../input-test-2");

    #[test]
    fn test_parse1() -> anyhow::Result<()> {
        dbg!(parse(INPUT1)?);
        Ok(())
    }

    #[test]
    fn test_parse2() -> anyhow::Result<()> {
        dbg!(parse(INPUT2)?);
        Ok(())
    }
}
