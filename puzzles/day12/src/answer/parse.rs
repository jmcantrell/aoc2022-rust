use anyhow::Context;

use aoc::Input;

use crate::core::HeightMap;

pub type Parsed = HeightMap;

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
    input.try_into().context("unable to parse height map")
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
