use aoc::Input;

use crate::core::DataStream;

pub type Parsed = DataStream;

pub fn parse(input: Input) -> anyhow::Result<DataStream> {
    Ok(input.chars().collect())
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn parse() -> anyhow::Result<()> {
        for input in INPUT.lines().map(|s| s.split_whitespace().next().unwrap()) {
            dbg!(super::parse(input)?);
        }
        Ok(())
    }
}
