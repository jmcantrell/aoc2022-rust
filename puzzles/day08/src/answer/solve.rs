use anyhow::Context;

use super::{Parsed1, Parsed2};

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(tree_patch: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(tree_patch.visibility().count())
}

pub fn solve2(tree_patch: &Parsed2) -> anyhow::Result<Solution2> {
    tree_patch
        .scenic_scores()
        .map(|(_, score)| score)
        .max()
        .context("grid is empty")
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

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
