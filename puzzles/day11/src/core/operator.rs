use anyhow::anyhow;

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Multiply,
}

impl TryFrom<&str> for Operator {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Multiply),
            _ => Err(anyhow!("invalid operator: {:?}", s)),
        }
    }
}
