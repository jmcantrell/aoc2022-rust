use aoc::Input;

type Parsed = usize;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    todo!()
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    todo!()
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_parse1() -> anyhow::Result<()> {
        dbg!(parse1(INPUT)?);
        Ok(())
    }

    #[test]
    fn test_parse2() -> anyhow::Result<()> {
        dbg!(parse2(INPUT)?);
        Ok(())
    }
}
