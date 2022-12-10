use anyhow::Context;
use aoc::Parser;

pub type Parsed = Vec<Vec<usize>>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse_elves(s: &str) -> anyhow::Result<Parsed> {
    let parse_item = |s: &str| {
        s.parse()
            .with_context(|| format!("invalid integer: {:?}", s))
    };

    let parse_items = |s: &str| {
        s.lines()
            .enumerate()
            .map(|(i, s)| parse_item(s).with_context(|| format!("item number {}", i + 1)))
            .collect::<Result<Vec<_>, _>>()
    };

    s.split("\n\n")
        .enumerate()
        .map(|(i, s)| parse_items(s).with_context(|| format!("elf number {}", i + 1)))
        .collect::<Result<Vec<Vec<_>>, _>>()
        .context("unable to parse elves")
}

#[derive(Debug, Clone)]
pub struct Parser1<'i>(pub &'i str);

impl Parser for Parser1<'_> {
    type Parsed = anyhow::Result<Parsed1>;

    fn parse(&self) -> Self::Parsed {
        parse_elves(self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Parser2<'i>(pub &'i str);

impl Parser for Parser2<'_> {
    type Parsed = anyhow::Result<Parsed2>;

    fn parse(&self) -> Self::Parsed {
        parse_elves(self.0)
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
