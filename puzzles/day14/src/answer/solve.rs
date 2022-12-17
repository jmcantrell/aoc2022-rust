use crate::core::Location;

use super::{Parsed1, Parsed2};

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

const START: Location = Location {
    row: 0,
    column: 500,
};

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(parsed.clone().fill_with_sand(&START).count())
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    let mut cave = parsed.clone();

    cave.floor = Some(cave.lowest_rock + 2);

    Ok(cave.fill_with_sand(&START).count())
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 24);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 93);
        Ok(())
    }
}
