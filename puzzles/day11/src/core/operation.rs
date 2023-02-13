use std::convert::TryFrom;

use anyhow::{ensure, Context};

use crate::core::{binary, prefix, Operator, Value};

const NEW: &str = "new";

#[derive(Debug, Clone)]
pub struct Operation {
    operand1: Value,
    operand2: Value,
    operator: Operator,
}

impl Operation {
    pub fn eval(&self, parameter: usize) -> usize {
        let value1 = self.operand1.unwrap(parameter);
        let value2 = self.operand2.unwrap(parameter);

        match self.operator {
            Operator::Add => value1 + value2,
            Operator::Multiply => value1 * value2,
        }
    }
}

impl TryFrom<&str> for Operation {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> anyhow::Result<Operation> {
        fn parse_operand(s: &str) -> anyhow::Result<Value> {
            s.try_into()
                .with_context(|| format!("invalid value: {s:?}"))
        }

        let s = prefix(s, "Operation:")?;

        let (lhs, rhs) = binary(s, "=")?;

        ensure!(
            lhs == NEW,
            "expected the left hand side of the operation to be: {:?}",
            NEW
        );

        let mut words = rhs.split_whitespace();

        let operand1 = parse_operand(words.next().context("missing first operand")?)?;
        let operator = words.next().context("missing operator")?.try_into()?;
        let operand2 = parse_operand(words.next().context("missing second operand")?)?;

        Ok(Self {
            operand1,
            operator,
            operand2,
        })
    }
}
