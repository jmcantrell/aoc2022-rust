use aoc::Input;

use crate::core::TreePatch;

pub type Parsed = TreePatch;

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
    TreePatch::try_from(input)
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
