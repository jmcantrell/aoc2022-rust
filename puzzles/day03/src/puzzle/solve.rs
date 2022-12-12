use anyhow::Context;

use crate::core::{find_common_by_chunk, find_common_by_pocket};
use crate::puzzle::{Parsed1, Parsed2};

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(find_common_by_pocket(parsed)
        .context("no common priority")?
        .into_iter()
        .sum())
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    Ok(find_common_by_chunk(parsed, 3)
        .context("no common priority")?
        .into_iter()
        .sum())
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::puzzle::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 157);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 70);
        Ok(())
    }
}
