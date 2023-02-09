use anyhow::anyhow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rotation {
    Left,
    Right,
}

impl Rotation {
    pub fn inverse(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl TryFrom<&str> for Rotation {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(anyhow!("invalid rotation: {:?}", s)),
        }
    }
}
