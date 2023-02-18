use super::Parsed;

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(tree_patch: &Parsed) -> anyhow::Result<Solution1> {
    Ok(tree_patch.count_visible())
}

pub fn solve2(tree_patch: &Parsed) -> anyhow::Result<Solution2> {
    Ok(tree_patch.max_scenic_score())
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::parse;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse(INPUT)?)?, 21);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse(INPUT)?)?, 8);
        Ok(())
    }
}
