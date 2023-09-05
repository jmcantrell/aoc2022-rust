use anyhow::Context;

use crate::core::sum_snacks_by_elf;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(elves: &Parsed1) -> anyhow::Result<Solution1> {
    sum_snacks_by_elf(elves).max().context("no elves")
}

pub fn solve2(elves: &Parsed2) -> anyhow::Result<Solution2> {
    let mut sums: Vec<_> = sum_snacks_by_elf(elves).collect();
    sums.sort();
    sums.reverse();
    Ok(sums.into_iter().take(3).sum())
}

#[cfg(test)]
mod tests {
    use crate::answer::{parse1, parse2};

    const INPUT: &str = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 24000);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 45000);
        Ok(())
    }
}
