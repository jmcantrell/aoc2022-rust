use super::Parse;

pub trait Solve<P: Parse> {
    type Solution: std::fmt::Debug;

    fn new(parsed: P::Parsed) -> Self;

    fn solve(&self) -> anyhow::Result<Self::Solution>;
}
