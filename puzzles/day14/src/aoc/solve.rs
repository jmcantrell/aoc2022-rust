use anyhow::Context;

use aoc::{Parse, Solve};

use crate::answer::{solve1, solve2, Solution1, Solution2};

use super::{Parser1, Parser2};

#[derive(Debug, Clone)]
pub struct Solver1(pub <Parser1 as Parse>::Parsed);

impl Solve<Parser1> for Solver1 {
    type Solution = Solution1;

    fn new(parsed: <Parser1 as Parse>::Parsed) -> Self {
        Self(parsed)
    }

    fn solve(&self) -> anyhow::Result<Self::Solution> {
        solve1(&self.0).context("solution failed for problem number 1")
    }
}

#[derive(Debug, Clone)]
pub struct Solver2(pub <Parser2 as Parse>::Parsed);

impl Solve<Parser2> for Solver2 {
    type Solution = Solution2;

    fn new(parsed: <Parser2 as Parse>::Parsed) -> Self {
        Self(parsed)
    }

    fn solve(&self) -> anyhow::Result<Self::Solution> {
        solve2(&self.0).context("solution failed for problem number 2")
    }
}
