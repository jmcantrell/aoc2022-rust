use std::convert::TryFrom;

use anyhow::Context;

use crate::core::prefix;

#[derive(Debug, Clone)]
pub struct Test {
    pub divisible_by: usize,
    pub if_true: usize,
    pub if_false: usize,
}

impl Test {
    pub fn eval(&self, value: usize) -> usize {
        if value % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

impl TryFrom<&str> for Test {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut lines = s.lines();

        let divisible_by = prefix(lines.next().context("missing test")?, "Test: divisible by")?;

        let divisible_by = divisible_by
            .parse()
            .with_context(|| format!("invalid condition: {divisible_by:?}"))?;

        let if_true = prefix(
            lines.next().context("missing true branch")?,
            "If true: throw to monkey",
        )?;

        let if_true = if_true
            .parse()
            .with_context(|| format!("invalid true branch: {if_true:?}"))?;

        let if_false = prefix(
            lines.next().context("missing false branch")?,
            "If false: throw to monkey",
        )?;

        let if_false = if_false
            .parse()
            .with_context(|| format!("invalid false branch: {if_false:?}"))?;

        Ok(Self {
            divisible_by,
            if_true,
            if_false,
        })
    }
}
