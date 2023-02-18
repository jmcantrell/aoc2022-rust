use crate::core::Value;

use super::Parsed;

pub type Solution = Value;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(parsed: &Parsed) -> anyhow::Result<Solution1> {
    Ok(parsed.eval())
}

pub fn solve2(parsed: &Parsed) -> anyhow::Result<Solution2> {
    Ok(parsed.eval_variable())
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::parse;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse(INPUT)?)?, 152);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse(INPUT)?)?, 301);
        Ok(())
    }
}
