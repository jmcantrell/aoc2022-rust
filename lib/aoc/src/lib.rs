use std::fmt::Debug;

pub type Input = &'static str;

pub trait Parse {
    type Parsed: Debug;

    fn new(input: Input) -> Self;

    fn parse(&self) -> anyhow::Result<Self::Parsed>;
}

pub trait Solve<P: Parse> {
    type Solution: Debug;

    fn new(parsed: P::Parsed) -> Self;

    fn solve(&self) -> anyhow::Result<Self::Solution>;
}
