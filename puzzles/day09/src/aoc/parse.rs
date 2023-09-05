use anyhow::Context;

use aoc::{Input, Parse};

use crate::answer::{parse1, parse2, Parsed1, Parsed2};

#[derive(Debug, Clone)]
pub struct Parser1(pub Input);

impl Parse for Parser1 {
    type Parsed = Parsed1;

    fn new(input: Input) -> Self {
        Self(input)
    }

    fn parse(&self) -> anyhow::Result<Self::Parsed> {
        parse1(self.0).context("parsing failed for problem number 1")
    }
}

#[derive(Debug, Clone)]
pub struct Parser2(pub Input);

impl Parse for Parser2 {
    type Parsed = Parsed2;

    fn new(input: Input) -> Self {
        Self(input)
    }

    fn parse(&self) -> anyhow::Result<Self::Parsed> {
        parse2(self.0).context("parsing failed for problem number 2")
    }
}
