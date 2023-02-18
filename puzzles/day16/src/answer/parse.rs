use aoc::Input;

use crate::core::RoomGraph;

pub type Parsed = RoomGraph<'static>;

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
    RoomGraph::try_from(input)
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn parse() -> anyhow::Result<()> {
        dbg!(super::parse(INPUT)?);
        Ok(())
    }
}
