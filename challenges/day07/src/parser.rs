use crate::terminal::Output;
use anyhow::Context;
use aoc::Parser;

pub type Parsed<'i> = Vec<Output<'i>>;
pub type Parsed1<'i> = Parsed<'i>;
pub type Parsed2<'i> = Parsed<'i>;

pub fn parse_terminal_output(input: &str) -> anyhow::Result<Parsed> {
    input
        .lines()
        .enumerate()
        .filter_map(|(i, s)| {
            if s.trim().len() > 0 {
                Some(Output::try_from(s).with_context(|| format!("line number {}", i + 1)))
            } else {
                None
            }
        })
        .collect::<Result<Vec<_>, _>>()
        .context("unable to parse terminal output")
}

#[derive(Debug, Clone)]
pub struct Parser1<'i>(pub &'i str);

impl<'i> Parser for Parser1<'i> {
    type Parsed = anyhow::Result<Parsed1<'i>>;

    fn parse(&self) -> Self::Parsed {
        parse_terminal_output(self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Parser2<'i>(pub &'i str);

impl<'i> Parser for Parser2<'i> {
    type Parsed = anyhow::Result<Parsed2<'i>>;

    fn parse(&self) -> Self::Parsed {
        parse_terminal_output(self.0)
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
