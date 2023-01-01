use std::convert::TryFrom;

use anyhow::anyhow;

#[derive(Debug, Clone, Copy)]
pub enum JetPush {
    Left,
    Right,
}

impl TryFrom<char> for JetPush {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(anyhow!("invalid character: {:?}", c)),
        }
    }
}
