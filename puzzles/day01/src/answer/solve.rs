use anyhow::Context;

use crate::core::sum_snacks_by_elf;

use super::Parsed;

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(elves: &Parsed) -> anyhow::Result<Solution1> {
    sum_snacks_by_elf(elves)
        .into_iter()
        .max()
        .context("no elves")
}

pub fn solve2(elves: &Parsed) -> anyhow::Result<Solution2> {
    let mut sums = sum_snacks_by_elf(elves);
    sums.sort();
    sums.reverse();
    Ok(sums.into_iter().take(3).sum())
}

#[cfg(test)]
mod tests {
    use crate::answer::parse;

    const INPUT: &str = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse(INPUT)?)?, 24000);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse(INPUT)?)?, 45000);
        Ok(())
    }
}
