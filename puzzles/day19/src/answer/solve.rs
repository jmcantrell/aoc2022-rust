use rayon::prelude::*;

use super::Parsed;

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(parsed: &Parsed) -> anyhow::Result<Solution1> {
    Ok(parsed
        .par_iter()
        .map(|blueprint| blueprint.max_geodes_collectable(24) as usize * blueprint.id as usize)
        .sum())
}

pub fn solve2(parsed: &Parsed) -> anyhow::Result<Solution2> {
    Ok(parsed
        .par_iter()
        .take(3)
        .map(|blueprint| blueprint.max_geodes_collectable(32) as usize)
        .product())
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::parse;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse(INPUT)?)?, 33);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse(INPUT)?)?, 3472);
        Ok(())
    }
}
