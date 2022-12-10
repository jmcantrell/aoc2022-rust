use crate::datastream::DataStream;
use aoc::Parser;

pub type Parsed = DataStream;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(s: &str) -> DataStream {
    s.chars().collect()
}

#[derive(Debug, Clone)]
pub struct Parser1<'i>(pub &'i str);

impl Parser for Parser1<'_> {
    type Parsed = anyhow::Result<Parsed1>;

    fn parse(&self) -> Self::Parsed {
        Ok(parse(self.0))
    }
}

#[derive(Debug, Clone)]
pub struct Parser2<'i>(pub &'i str);

impl Parser for Parser2<'_> {
    type Parsed = anyhow::Result<Parsed2>;

    fn parse(&self) -> Self::Parsed {
        Ok(parse(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../input-test.txt");

    fn inputs() -> Vec<&'static str> {
        INPUT
            .lines()
            .map(|s| s.split_whitespace().next().unwrap())
            .collect()
    }

    #[test]
    fn parse1() -> anyhow::Result<()> {
        for input in inputs() {
            Parser1(input).parse()?;
        }
        Ok(())
    }

    #[test]
    fn parse2() -> anyhow::Result<()> {
        for input in inputs() {
            Parser2(input).parse()?;
        }
        Ok(())
    }
}
