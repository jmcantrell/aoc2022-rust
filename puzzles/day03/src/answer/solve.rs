use anyhow::Context;

use crate::core::{find_common_by_chunk, find_common_by_pocket};

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(rucksacks: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(find_common_by_pocket(rucksacks)
        .context("no common priority")?
        .into_iter()
        .sum())
}

pub fn solve2(rucksacks: &Parsed2) -> anyhow::Result<Solution2> {
    Ok(find_common_by_chunk(rucksacks, 3)
        .context("no common priority")?
        .into_iter()
        .sum())
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 157);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 70);
        Ok(())
    }
}
