use crate::core::Location;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

const START: Location = Location::new(500, 0);

pub fn solve1(map: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(map.clone().fill_with_sand(&START).count())
}

pub fn solve2(map: &Parsed2) -> anyhow::Result<Solution2> {
    let mut map = map.clone();

    map.floor = Some(map.lowest_rock + 2);

    Ok(map.fill_with_sand(&START).count())
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 24);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 93);
        Ok(())
    }
}
