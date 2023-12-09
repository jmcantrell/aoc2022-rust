use anyhow::Context;

use aoc::Input;

use crate::core::{Movement, Procedure, Stacks};

type Parsed = (Stacks, Procedure);
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    let mut chunks = input.split("\n\n");

    let stacks = Stacks::try_from(chunks.next().context("missing stacks")?)?;

    let movements = chunks
        .next()
        .context("missing procedure")?
        .lines()
        .map(Movement::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    Ok((stacks, Procedure(movements)))
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

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        dbg!(parse(INPUT)?);
        Ok(())
    }
}
