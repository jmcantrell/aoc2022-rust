use anyhow::anyhow;

use super::RelativeDirection;

use Movement::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Movement {
    Forward(usize),
    Rotate(RelativeDirection),
}

impl TryFrom<&str> for Movement {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Ok(value) = s.parse::<usize>() {
            Ok(Forward(value))
        } else {
            match s {
                "L" => Ok(Rotate(RelativeDirection::Left)),
                "R" => Ok(Rotate(RelativeDirection::Right)),
                _ => Err(anyhow!("invalid movement: {:?}", s)),
            }
        }
    }
}
