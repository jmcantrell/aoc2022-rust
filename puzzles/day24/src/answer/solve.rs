use crate::core::find_fastest_time;

use super::{Parsed1, Parsed2};

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(map: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(find_fastest_time(map, &[map.start, map.end]))
}

pub fn solve2(map: &Parsed2) -> anyhow::Result<Solution2> {
    Ok(find_fastest_time(map, &[map.start, map.end, map.start, map.end]))
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 18);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 54);
        Ok(())
    }
}
