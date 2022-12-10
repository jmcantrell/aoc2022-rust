use crate::parser::{Parsed1, Parsed2};
use aoc::Solver;

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

#[derive(Debug, Clone)]
pub struct Solver1(pub Parsed1);

impl Solver for Solver1 {
    type Solution = anyhow::Result<Solution1>;

    fn solve(&self) -> Self::Solution {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Solver2(pub Parsed2);

impl Solver for Solver2 {
    type Solution = anyhow::Result<Solution2>;

    fn solve(&self) -> Self::Solution {
        todo!()
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
        assert_eq!(Solver1(Parser1(INPUT).parse()?).solve()?, 0);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(Solver2(Parser2(INPUT).parse()?).solve()?, 0);
        Ok(())
    }
}
