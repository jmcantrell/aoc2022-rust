use anyhow::Context;

use aoc::{Input, Parse};

use crate::answer::{parse, Parsed};

#[derive(Debug, Clone)]
pub struct Parser(pub Input);

impl Parse for Parser {
    type Parsed = Parsed;

    fn new(input: Input) -> Self {
        Self(input)
    }

    fn parse(&self) -> anyhow::Result<Self::Parsed> {
        parse(self.0).context("parsing failed for problem number 1")
    }
}
