use anyhow::Context;

use crate::core::{ScenicScores, Visibility};

use super::{Parsed1, Parsed2};

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(parsed.visibility().visible().count())
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    Ok(*parsed
        .scenic_scores()
        .values()
        .max()
        .context("grid is empty")?)
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::puzzle::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 21);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 8);
        Ok(())
    }
}
