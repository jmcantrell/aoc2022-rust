use aoc::Input;

use crate::core::DataStream;

type Parsed = DataStream;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<DataStream> {
    Ok(input.chars().collect())
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
        for input in INPUT.lines().map(|s| s.split_whitespace().next().unwrap()) {
            dbg!(parse(input)?);
        }
        Ok(())
    }
}
