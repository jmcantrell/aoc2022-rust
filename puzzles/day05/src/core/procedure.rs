use super::{Crane, Movement, Stacks};

#[derive(Debug, Clone)]
pub struct Procedure(pub Vec<Movement>);

impl Procedure {
    pub fn execute<C: Crane>(&self, stacks: &mut Stacks) -> anyhow::Result<()> {
        for movement in self.0.iter() {
            movement.execute::<C>(stacks)?;
        }
        Ok(())
    }
}
