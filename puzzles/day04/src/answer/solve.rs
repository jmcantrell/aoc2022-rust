use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(pairs: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(pairs.iter().filter(|pair| pair.has_redundancy()).count())
}

pub fn solve2(pairs: &Parsed2) -> anyhow::Result<Solution2> {
    Ok(pairs.iter().filter(|pair| pair.has_overlap()).count())
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 2);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 4);
        Ok(())
    }
}
