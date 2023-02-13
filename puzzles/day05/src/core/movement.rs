use anyhow::{anyhow, ensure, Context};

use super::{Crane, Stacks};

#[derive(Debug, Clone)]
pub struct Movement {
    count: usize,
    from: usize,
    to: usize,
}

impl Movement {
    pub fn execute<C: Crane>(&self, stacks: &mut Stacks) -> anyhow::Result<()> {
        C::move_crates(stacks, self.count, self.from - 1, self.to - 1)
            .with_context(|| format!("unable to execute command: {self:?}"))
    }
}

impl TryFrom<&str> for Movement {
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

                Ok(Movement { count, from, to })
            }
            _ => Err(anyhow!("unknown command: {:?}", command)),
        }
    }
}
