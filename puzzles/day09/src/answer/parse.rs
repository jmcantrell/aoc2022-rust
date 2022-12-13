use anyhow::Context;

use aoc::Input;

use crate::core::Vector;

pub type Parsed = Vec<Vector>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    fn parse_vector(s: &str) -> anyhow::Result<Vector> {
        s.try_into()
            .with_context(|| format!("invalid vector: {:?}", s))
    }

    input
        .lines()
        .enumerate()
        .map(|(i, s)| parse_vector(s).with_context(|| format!("line number {}", i + 1)))
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

    const INPUT1: Input = include_str!("../../input-test-1.txt");
    const INPUT2: Input = include_str!("../../input-test-2.txt");

    #[test]
    fn parse1() -> anyhow::Result<()> {
        dbg!(super::parse1(INPUT1)?);
        Ok(())
    }

    #[test]
    fn parse2() -> anyhow::Result<()> {
        dbg!(super::parse2(INPUT2)?);
        Ok(())
    }
}
