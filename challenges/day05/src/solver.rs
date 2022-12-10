use crate::crane::{Crane, Crane9000, Crane9001};
use crate::parser::{Parsed1, Parsed2, Procedure};
use crate::ship::Stacks;
use anyhow::Context;
use aoc::Solver;

pub type Solution = String;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

#[derive(Debug, Clone)]
pub struct Solver1(pub Parsed1);

fn execute<C: Crane>(procedure: &Procedure) -> anyhow::Result<Stacks> {
    let mut stacks = procedure.stacks.clone();

    procedure
        .commands
        .iter()
        .enumerate()
        .map(|(i, command)| -> anyhow::Result<()> {
            C::execute_command(&mut stacks, &command)
                .with_context(|| format!("command number {} failed", i + 1))
        })
        .collect::<Result<Vec<_>, _>>()
        .context("unable to complete procedure")?;

    Ok(stacks)
}

fn get_message(stacks: &Stacks) -> String {
    stacks
        .top()
        .into_iter()
        .map(|c| c.unwrap_or(&' '))
        .collect()
}

impl Solver for Solver1 {
    type Solution = anyhow::Result<Solution1>;

    fn solve(&self) -> Self::Solution {
        Ok(get_message(&execute::<Crane9000>(&self.0)?))
    }
}

#[derive(Debug, Clone)]
pub struct Solver2(pub Parsed2);

impl Solver for Solver2 {
    type Solution = anyhow::Result<Solution2>;

    fn solve(&self) -> Self::Solution {
        Ok(get_message(&execute::<Crane9001>(&self.0)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Parser1, Parser2};
    use aoc::Parser;

    const INPUT: &'static str = include_str!("../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(Solver1(Parser1(INPUT).parse()?).solve()?, "CMZ");
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(Solver2(Parser2(INPUT).parse()?).solve()?, "MCD");
        Ok(())
    }
}
