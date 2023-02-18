use anyhow::Context;

use aoc::Input;

use crate::core::Movement;

pub type Parsed = Vec<Movement>;

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
    input
        .lines()
        .enumerate()
        .map(|(i, s)| {
            s.try_into()
                .with_context(|| format!("line number {}", i + 1))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    const INPUT1: Input = include_str!("../../input-test-1.txt");
    const INPUT2: Input = include_str!("../../input-test-2.txt");

    #[test]
    fn parse1() -> anyhow::Result<()> {
        dbg!(super::parse(INPUT1)?);
        Ok(())
    }

    #[test]
    fn parse2() -> anyhow::Result<()> {
        dbg!(super::parse(INPUT2)?);
        Ok(())
    }
}
