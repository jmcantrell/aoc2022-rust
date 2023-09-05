use crate::core::{decode_message, Crane9000, Crane9001, Message};

use super::{Parsed1, Parsed2};

type Solution = Message;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    let (stacks, procedure) = parsed;
    decode_message::<Crane9000>(stacks, procedure)
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    let (stacks, procedure) = parsed;
    decode_message::<Crane9001>(stacks, procedure)
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, "CMZ");
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, "MCD");
        Ok(())
    }
}
