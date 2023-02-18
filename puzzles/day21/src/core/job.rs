use anyhow::Context;

use crate::core::Operation;

use super::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Job<'a> {
    Value(Value),
    Operation(Operation, &'a str, &'a str),
}

impl<'a> TryFrom<&'a str> for Job<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        if let Ok(value) = s.parse::<Value>() {
            Ok(Self::Value(value))
        } else {
            let mut split = s.split_whitespace();
            let name1 = split.next().context("missing first name")?;
            let operation: Operation = split.next().context("missing operator")?.try_into()?;
            let name2 = split.next().context("missing second name")?;
            Ok(Self::Operation(operation, name1, name2))
        }
    }
}
