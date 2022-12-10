use aoc::Parser;

pub type Parsed = usize;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

#[derive(Debug, Clone)]
pub struct Parser1<'i>(pub &'i str);

impl Parser for Parser1<'_> {
    type Parsed = anyhow::Result<Parsed1>;

    fn parse(&self) -> Self::Parsed {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Parser2<'i>(pub &'i str);

impl Parser for Parser2<'_> {
    type Parsed = anyhow::Result<Parsed2>;

    fn parse(&self) -> Self::Parsed {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../input-test.txt");

    #[test]
    fn parse1() -> anyhow::Result<()> {
        dbg!(Parser1(INPUT).parse()?);
        Ok(())
    }

    #[test]
    fn parse2() -> anyhow::Result<()> {
        dbg!(Parser2(INPUT).parse()?);
        Ok(())
    }
}
