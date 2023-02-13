use aoc::Input;

use crate::core::Grove;

pub type Parsed = Grove;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    input.try_into()
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    input.try_into()
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn parse1() -> anyhow::Result<()> {
        dbg!(super::parse1(INPUT)?);
        Ok(())
    }

    #[test]
    fn parse2() -> anyhow::Result<()> {
        dbg!(super::parse2(INPUT)?);
        Ok(())
    }
}
