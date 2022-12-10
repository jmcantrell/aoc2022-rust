use anyhow::{anyhow, Context};
use aoc::Parser;

pub type Parsed = Vec<Vec<usize>>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse_rucksacks(input: &str) -> anyhow::Result<Parsed> {
    let parse_priority = |c| {
        match c {
            'a'..='z' => Ok('a' as usize - 1),
            'A'..='Z' => Ok('A' as usize - 26 - 1),
            _ => Err(anyhow!("unrecognized character: {:?}", c)),
        }
        .map(|offset| c as usize - offset)
    };

    let parse_rucksack = |s: &str| {
        s.chars()
            .enumerate()
            .map(|(i, c)| parse_priority(c).with_context(|| format!("column number {}", i + 1)))
            .collect::<Result<Vec<_>, _>>()
    };

    input
        .lines()
        .enumerate()
        .map(|(i, s)| parse_rucksack(s).with_context(|| format!("line number {}", i + 1)))
        .collect::<Result<Vec<_>, _>>()
        .context("unable to parse rucksacks")
}

#[derive(Debug, Clone)]
pub struct Parser1<'i>(pub &'i str);

impl<'i> Parser for Parser1<'i> {
    type Parsed = anyhow::Result<Parsed1>;

    fn parse(&self) -> Self::Parsed {
        parse_rucksacks(self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Parser2<'i>(pub &'i str);

impl<'i> Parser for Parser2<'i> {
    type Parsed = anyhow::Result<Parsed2>;

    fn parse(&self) -> Self::Parsed {
        parse_rucksacks(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../input-test.txt");

    #[test]
    fn parse1() -> anyhow::Result<()> {
        dbg!(Parser1(INPUT).parse()?);
        Ok(())
    }

    #[test]
    fn parse2() -> anyhow::Result<()> {
        dbg!(Parser2(INPUT).parse()?);
        Ok(())
    }
}
