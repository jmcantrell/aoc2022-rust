use anyhow::Context;

use crate::core::find_unique_window;

use super::Parsed;

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(data: &Parsed) -> anyhow::Result<Solution1> {
    find_unique_window(data.iter(), 4).context("no start-of-packet marker detected")
}

pub fn solve2(data: &Parsed) -> anyhow::Result<Solution2> {
    find_unique_window(data.iter(), 14).context("no start-of-message marker detected")
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::parse;

    use super::{Solution1, Solution2};

    const INPUT: Input = include_str!("../../input-test.txt");

    fn test_cases() -> Vec<(&'static str, (Solution1, Solution2))> {
        INPUT
            .lines()
            .map(|s| {
                let mut words = s.split_whitespace();
                let input = words.next().unwrap();
                let expected1: usize = words.next().unwrap().parse().unwrap();
                let expected2: usize = words.next().unwrap().parse().unwrap();
                (input, (expected1, expected2))
            })
            .collect()
    }

    #[test]
    fn solve1() -> anyhow::Result<()> {
        for (input, (expected, _)) in test_cases() {
            assert_eq!(super::solve1(&parse(input)?)?, expected);
        }
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        for (input, (_, expected)) in test_cases() {
            assert_eq!(super::solve2(&parse(input)?)?, expected);
        }
        Ok(())
    }
}
