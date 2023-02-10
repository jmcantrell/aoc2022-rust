use anyhow::anyhow;

use geometry::Rotation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Movement {
    Forward(usize),
    Rotate(Rotation),
}

impl TryFrom<&str> for Movement {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Ok(value) = s.parse::<usize>() {
            Ok(Self::Forward(value))
        } else {
            match s {
                "L" => Ok(Self::Rotate(Rotation::Left)),
                "R" => Ok(Self::Rotate(Rotation::Right)),
                _ => Err(anyhow!("invalid movement: {:?}", s)),
            }
        }
    }
}
