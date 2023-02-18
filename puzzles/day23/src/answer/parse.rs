use aoc::Input;

use crate::core::Grove;

pub type Parsed = Grove;

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
    input.try_into()
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
