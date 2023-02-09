use crate::core::geometry::Rotation;

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
            Ok(Self::Rotate(Rotation::try_from(s)?))
        }
    }
}
