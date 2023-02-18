use anyhow::Context;

use aoc::Input;

use crate::core::SnafuNumber;

pub type Parsed = Vec<SnafuNumber>;

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
    input
        .lines()
        .enumerate()
        .map(|(i, s)| {
            s.try_into()
                .with_context(|| format!("line number {}", i + 1))
        })
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