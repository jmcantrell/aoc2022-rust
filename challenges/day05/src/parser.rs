use crate::command::Command;
use crate::ship::Stacks;
use anyhow::Context;
use aoc::Parser;

#[derive(Debug, Clone)]
pub struct Procedure {
    pub stacks: Stacks,
    pub commands: Vec<Command>,
}

pub type Parsed1 = Procedure;
pub type Parsed2 = Procedure;

fn parse_stacks_and_commands(s: &str) -> anyhow::Result<Procedure> {
    let mut chunks = s.split("\n\n");

    let stacks = Stacks::try_from(chunks.next().context("missing stacks")?)?;

    let commands = chunks
        .next()
        .context("missing procedure")?
        .lines()
        .map(Command::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Procedure { stacks, commands })
}

#[derive(Debug, Clone)]
pub struct Parser1<'i>(pub &'i str);

impl Parser for Parser1<'_> {
    type Parsed = anyhow::Result<Parsed1>;

    fn parse(&self) -> Self::Parsed {
        parse_stacks_and_commands(self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Parser2<'i>(pub &'i str);

impl Parser for Parser2<'_> {
    type Parsed = anyhow::Result<Parsed2>;

    fn parse(&self) -> Self::Parsed {
        parse_stacks_and_commands(self.0)
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
