use std::collections::HashSet;

use anyhow::ensure;

use crate::core::{Point, Rectangle, TaxicabCircle, coalesce_ranges};

use super::{Parsed1, Parsed2};

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

pub fn solve1(grid: &Parsed1) -> anyhow::Result<Solution1> {
    let mut rect = grid.extents();

    rect.top_left.y = ROW;
    rect.bottom_right.y = ROW;

    let beacons: HashSet<_> = grid.beacons().collect();
    let circles: Vec<_> = grid.taxicab_circles().collect();

    let no_beacons = rect.points().filter_map(move |point| {
        if !beacons.contains(&point) && circles.iter().any(|c| c.contains(&point)) {
            Some(point)
        } else {
            None
        }
    });

    Ok(no_beacons.count())
}

pub fn solve2(grid: &Parsed2) -> anyhow::Result<Solution2> {
    let rect = Rectangle {
        top_left: Point { x: 0, y: 0 },
        bottom_right: Point {
            x: MAX_COMPONENT,
            y: MAX_COMPONENT,
        },
    };

    let circles: Vec<TaxicabCircle> = grid.taxicab_circles().collect();

    let mut possible_beacons: Vec<_> = (rect.top()..=rect.bottom())
        .filter_map(|y| {
            let x_ranges = coalesce_ranges(circles.iter().filter_map(|c| c.x_range(y)).collect());
            if x_ranges.len() > 1 {
                Some((y, x_ranges))
            } else {
                None
            }
        })
        .collect();

    ensure!(possible_beacons.len() > 0, "no candidate rows found");

    ensure!(
        possible_beacons.len() == 1,
        "candidates found on multiple rows"
    );

    let (y, mut x_ranges) = possible_beacons.pop().unwrap();

    ensure!(x_ranges.len() > 0, "no candidates found on row {}", y);

    x_ranges.reverse();

    let x_range1 = x_ranges.pop().unwrap();

    let x = *x_range1.end() + 1;

    let distress_beacon = Point { x, y };

    Ok(distress_beacon.x * (4000000 as isize) + distress_beacon.y)
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 26);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 56_000_011);
        Ok(())
    }
}
