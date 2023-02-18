use anyhow::Context;

use aoc::Input;

use crate::core::JetPush;

pub type Parsed = Vec<JetPush>;

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
    input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            c.try_into()
                .with_context(|| format!("jet push number {}", i + 1))
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
