use anyhow::Context;

const OLD: &str = "old";

#[derive(Debug, Clone)]
pub enum Value {
    Parameter,
    Const(usize),
}

impl Value {
    pub fn unwrap(&self, parameter: usize) -> usize {
        match self {
            Value::Parameter => parameter,
            Value::Const(value) => *value,
        }
    }
}

impl TryFrom<&str> for Value {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            OLD => Ok(Value::Parameter),
            _ => Ok(Value::Const(
                s.parse()
                    .with_context(|| format!("invalid integer: {s:?}"))?,
            )),
        }
    }
}
