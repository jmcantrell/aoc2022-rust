use anyhow::Context;

use aoc::Input;

use crate::core::Value;

pub type Parsed = Vec<Value>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    input
        .lines()
        .enumerate()
        .map(|(i, s)| s.parse().with_context(|| format!("line number {}", i + 1)))
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

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn parse1() -> anyhow::Result<()> {
        dbg!(super::parse1(INPUT)?);
        Ok(())
    }

    #[test]
    fn parse2() -> anyhow::Result<()> {
        dbg!(super::parse2(INPUT)?);
        Ok(())
    }
}
