use super::{Parsed1, Parsed2};

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(grove: &Parsed1) -> anyhow::Result<Solution1> {
    let mut grove = grove.clone();

    for _ in 0..10 {
        grove.iterate();
    }

    Ok(grove.count_empty_tiles())
}

pub fn solve2(grove: &Parsed2) -> anyhow::Result<Solution2> {
    let mut grove = grove.clone();
    let mut round = 1;

    while grove.iterate() {
        round += 1;
    }

    Ok(round)
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 110);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 20);
        Ok(())
    }
}
