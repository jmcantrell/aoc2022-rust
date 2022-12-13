use aoc::Input;

use crate::core::DataStream;

pub type Parsed = DataStream;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(s: &str) -> DataStream {
    s.chars().collect()
}

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    Ok(parse(input))
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    Ok(parse(input))
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    const INPUT: Input = include_str!("../../input-test.txt");

    fn inputs() -> Vec<Input> {
        INPUT
            .lines()
            .map(|s| s.split_whitespace().next().unwrap())
            .collect()
    }

    #[test]
    fn parse1() -> anyhow::Result<()> {
        for input in inputs() {
            dbg!(super::parse1(input)?);
        }
        Ok(())
    }

    #[test]
    fn parse2() -> anyhow::Result<()> {
        for input in inputs() {
            dbg!(super::parse2(input)?);
        }
        Ok(())
    }
}
