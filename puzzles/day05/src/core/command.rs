use anyhow::{anyhow, ensure, Context};

use super::{Crane, Stacks};

#[derive(Debug, Clone)]
pub enum Command {
    Move {
        count: usize,
        from: usize,
        to: usize,
    },
}

impl Command {
    pub fn execute<C: Crane>(&self, stacks: &mut Stacks) -> anyhow::Result<()> {
        match self {
            Command::Move { count, from, to } => C::move_crates(stacks, *count, *from - 1, *to - 1),
        }
        .with_context(|| format!("unable to execute command: {:?}", self))
    }
}

impl TryFrom<&str> for Command {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut words = s.split_whitespace();

        let command = words.next().context("missing command")?;

        match command {
            "move" => {
                let count: usize = words.next().context("missing count")?.parse()?;

                let from = words.next().context("missing 'from' subcommand")?;
                ensure!(from == "from", "unrecognized 'from' subcommand: {:?}", from);

                let from: usize = words
                    .next()
                    .context("missing argument for 'from' subcommand")?
                    .parse()?;

                let to = words.next().context("missing 'to' subcommand")?;
                ensure!(to == "to", "unrecognized 'to' subcommand: {:?}", to);

                let to = words
                    .next()
                    .context("missing argument for 'to' subcommand")?
                    .parse()?;

                Ok(Command::Move { count, from, to })
            }
            _ => Err(anyhow!("unknown command: {:?}", command)),
        }
    }
}
