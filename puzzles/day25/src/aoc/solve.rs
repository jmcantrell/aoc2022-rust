use anyhow::Context;

use aoc::{Parse, Solve};

use crate::answer::{solve, Solution};

use super::Parser;

#[derive(Debug, Clone)]
pub struct Solver(pub <Parser as Parse>::Parsed);

impl Solve<Parser> for Solver {
    type Solution = Solution;

    fn new(parsed: <Parser as Parse>::Parsed) -> Self {
        Self(parsed)
    }

    fn solve(&self) -> anyhow::Result<Self::Solution> {
        solve(&self.0).context("solution failed")
    }
}
