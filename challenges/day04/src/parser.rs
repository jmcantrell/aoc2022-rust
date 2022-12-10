use crate::assignment::AssignmentPair;
use anyhow::Context;
use aoc::Parser;

pub type Parsed = Vec<AssignmentPair>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse_assignment_pairs(s: &str) -> anyhow::Result<Parsed> {
    s.lines()
        .enumerate()
        .map(|(i, s)| {
            s.try_into()
                .with_context(|| format!("line number {}", i + 1))
        })
        .collect::<Result<Vec<_>, _>>()
        .context("unable to parse assignment pairs")
}

#[derive(Debug, Clone)]
pub struct Parser1<'i>(pub &'i str);

impl Parser for Parser1<'_> {
    type Parsed = anyhow::Result<Parsed1>;

    fn parse(&self) -> Self::Parsed {
        parse_assignment_pairs(self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Parser2<'i>(pub &'i str);

impl Parser for Parser2<'_> {
    type Parsed = anyhow::Result<Parsed2>;

    fn parse(&self) -> Self::Parsed {
        parse_assignment_pairs(self.0)
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
