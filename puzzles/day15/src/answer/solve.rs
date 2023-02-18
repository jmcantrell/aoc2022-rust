use std::collections::HashSet;

use anyhow::ensure;

use crate::core::{coalesce_ranges, Point, TaxicabCircle};

use super::Parsed;

pub type Solution1 = usize;
pub type Solution2 = isize;

#[cfg(test)]
mod consts {
    pub const ROW: isize = 10;
    pub const MAX_COMPONENT: isize = 20;
}

#[cfg(not(test))]
mod consts {
    pub const ROW: isize = 2_000_000;
    pub const MAX_COMPONENT: isize = 4_000_000;
}

use consts::*;

pub fn solve1(grid: &Parsed) -> anyhow::Result<Solution1> {
    let (mut top_left, mut bottom_right) = grid.extents();

    top_left.y = ROW;
    bottom_right.y = ROW;

    let beacons: HashSet<_> = grid.beacons().collect();
    let circles: Vec<_> = grid.taxicab_circles().collect();

    Ok((top_left.y..=bottom_right.y)
        .flat_map(|y| {
            (top_left.x..=bottom_right.x)
                .map(move |x| Point::new(x, y))
                .filter(|point| {
                    !beacons.contains(&point) && circles.iter().any(|c| c.contains(point))
                })
        })
        .count())
}

pub fn solve2(grid: &Parsed) -> anyhow::Result<Solution2> {
    let top_left = Point::default();
    let bottom_right = Point::new(MAX_COMPONENT, MAX_COMPONENT);

    let circles: Vec<TaxicabCircle> = grid.taxicab_circles().collect();

    let mut possible_beacons: Vec<_> = (top_left.y..=bottom_right.y)
        .filter_map(|y| {
            let x_ranges = coalesce_ranges(circles.iter().filter_map(|c| c.x_range(y)).collect());
            if x_ranges.len() > 1 {
                Some((y, x_ranges))
            } else {
                None
            }
        })
        .collect();

    ensure!(!possible_beacons.is_empty(), "no candidate rows found");

    ensure!(
        possible_beacons.len() == 1,
        "candidates found on multiple rows"
    );

    let (y, mut x_ranges) = possible_beacons.pop().unwrap();

    ensure!(!x_ranges.is_empty(), "no candidates found on row {}", y);

    x_ranges.reverse();

    let x_range1 = x_ranges.pop().unwrap();

    let x = *x_range1.end() + 1;

    let distress_beacon = Point::new(x, y);

    Ok(distress_beacon.x * 4_000_000 + distress_beacon.y)
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::parse;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse(INPUT)?)?, 26);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse(INPUT)?)?, 56_000_011);
        Ok(())
    }
}
