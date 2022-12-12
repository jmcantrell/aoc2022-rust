use super::{Crane, Procedure, Stacks};

pub type Message = String;

pub fn decode_message<C: Crane>(stacks: &Stacks, procedure: &Procedure) -> anyhow::Result<Message> {
    let mut stacks = stacks.clone();

    procedure.execute::<C>(&mut stacks)?;

    Ok(stacks
        .top()
        .into_iter()
        .map(|c| c.unwrap_or(&' '))
        .collect())
}
