use anyhow::Context;

use aoc::Input;

use crate::core::{Command, Procedure, Stacks};

pub type Parsed = (Stacks, Procedure);
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    let mut chunks = input.split("\n\n");

    let stacks = Stacks::try_from(chunks.next().context("missing stacks")?)?;

    let commands = chunks
        .next()
        .context("missing procedure")?
        .lines()
        .map(Command::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    Ok((stacks, Procedure(commands)))
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
