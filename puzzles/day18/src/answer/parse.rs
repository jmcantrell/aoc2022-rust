use anyhow::Context;

use aoc::Input;

use crate::core::{CubeGrid, Point};

type Parsed = CubeGrid;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    fn parse_int(s: &str) -> anyhow::Result<isize> {
        s.parse().with_context(|| format!("invalid integer: {s:?}"))
    }

    fn parse_point(s: &str) -> anyhow::Result<Point> {
        let mut components = s
            .trim()
            .splitn(3, ',')
            .enumerate()
            .map(|(i, s)| parse_int(s).with_context(|| format!("component number {}", i + 1)));

        let x = components.next().context("missing x component")??;
        let y = components.next().context("missing y component")??;
        let z = components.next().context("missing z component")??;

        Ok(Point::new(x, y, z))
    }

    Ok(CubeGrid::from(
        input
            .lines()
            .enumerate()
            .map(|(i, s)| parse_point(s).with_context(|| format!("point number {}", i + 1)))
            .collect::<Result<Vec<_>, _>>()
            .context("unable to parse points")?,
    ))
}

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    parse(input)
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    parse(input)
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use super::*;

    const INPUT: Input = include_str!("../../input-test");

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        dbg!(parse(INPUT)?);
        Ok(())
    }
}
