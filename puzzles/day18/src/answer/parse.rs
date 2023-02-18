use anyhow::Context;

use aoc::Input;

use crate::core::{CubeGrid, Point};

pub type Parsed = CubeGrid;

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
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
