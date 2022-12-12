use anyhow::Context;

use super::{Command, Crane, Stacks};

#[derive(Debug, Clone)]
pub struct Procedure(pub Vec<Command>);

impl Procedure {
    pub fn execute<C: Crane>(&self, stacks: &mut Stacks) -> anyhow::Result<()> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, command)| -> anyhow::Result<()> {
                command
                    .execute::<C>(stacks)
                    .with_context(|| format!("command number {}", i + 1))
            })
            .collect::<Result<Vec<_>, _>>()
            .context("unable to execute procedure")?;

        Ok(())
    }
}
