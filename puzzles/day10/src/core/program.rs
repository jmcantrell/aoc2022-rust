use std::convert::TryFrom;

use anyhow::{anyhow, Context};

pub type Program = Vec<Command>;

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Noop,
    AddX(isize),
}

impl Command {
    pub fn cycles(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::AddX(_) => 2,
        }
    }
}

impl TryFrom<&str> for Command {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut words = s.split_whitespace();

        let command = words.next().context("missing command name")?;

        match command {
            "noop" => Ok(Self::Noop),
            "addx" => {
                let arg = words
                    .next()
                    .with_context(|| format!("{:?} command is missing it's argument", command))?;
                let value: isize = arg
                    .parse()
                    .with_context(|| format!("invalid integer: {:?}", arg))?;
                Ok(Self::AddX(value))
            }
            _ => Err(anyhow!("unrecognized command: {:?}", command)),
        }
    }
}
