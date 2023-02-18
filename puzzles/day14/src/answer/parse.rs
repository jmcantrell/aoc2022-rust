use anyhow::Context;

use aoc::Input;

use crate::core::CaveMap;

pub type Parsed = CaveMap;

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
    input.try_into().context("unable to parse cave map")
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
