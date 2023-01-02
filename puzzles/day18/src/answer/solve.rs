use anyhow::Context;

use super::{Parsed1, Parsed2};

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(droplet: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(droplet.surface_area())
}

pub fn solve2(droplet: &Parsed2) -> anyhow::Result<Solution2> {
    let air_box = droplet.bounding_box().context("droplet has no points")? + 1;
    let air_and_droplet = air_box.to_grid();
    let air = air_and_droplet.difference(droplet);
    let outside_air = air.explore(air_box.min).context("invalid starting point")?;
    let inside_air = air.difference(&outside_air);

    Ok(droplet.surface_area() - inside_air.surface_area())
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 64);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 58);
        Ok(())
    }
}
