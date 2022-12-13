use anyhow::Context;

use aoc::Input;

use crate::core::{Elf, Elves, Snack};

pub type Parsed = Elves;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(s: &str) -> anyhow::Result<Parsed> {
    fn parse_int(s: &str) -> anyhow::Result<usize> {
        s.parse()
            .with_context(|| format!("invalid integer: {:?}", s))
    }

    fn parse_snack(s: &str) -> anyhow::Result<Snack> {
        parse_int(s).context("invalid snack")
    }

    fn parse_elf(s: &str) -> anyhow::Result<Elf> {
        s.lines()
            .enumerate()
            .map(|(i, s)| parse_snack(s).with_context(|| format!("snack number {}", i + 1)))
            .collect()
    }

    fn parse_elves(s: &str) -> anyhow::Result<Elves> {
        s.split("\n\n")
            .enumerate()
            .map(|(i, s)| parse_elf(s).with_context(|| format!("elf number {}", i + 1)))
            .collect()
    }

    parse_elves(s).context("unable to parse elves")
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
