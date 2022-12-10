use std::fmt::Debug;

pub trait Parser {
    type Parsed: Debug;
    fn parse(&self) -> Self::Parsed;
}

pub trait Solver {
    type Solution: Debug;
    fn solve(&self) -> Self::Solution;
}
