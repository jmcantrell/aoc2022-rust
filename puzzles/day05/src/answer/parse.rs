use anyhow::Context;

use aoc::Input;

use crate::core::{Movement, Procedure, Stacks};

pub type Parsed = (Stacks, Procedure);

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
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
