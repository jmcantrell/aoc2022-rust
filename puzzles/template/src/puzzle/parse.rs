use aoc::Input;

pub type Parsed = usize;
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
