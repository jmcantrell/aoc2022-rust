use crate::parser::{Parsed1, Parsed2};
use anyhow::Context;
use aoc::Solver;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

fn find_unique_window<I, T>(iter: I, size: usize) -> Option<usize>
where
    I: Iterator<Item = T>,
    T: Clone + Eq + Hash,
{
    let mut window: VecDeque<T> = Default::default();
    let mut count: usize = 0;

    for c in iter {
        window.push_back(c);
        count += 1;

        if window.len() > size {
            window.pop_front().unwrap();
        }

        if window.len() == size {
            let set: HashSet<T> = HashSet::from_iter(window.iter().cloned());
            if set.len() == size {
                return Some(count);
            }
        }
    }

    None
}

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

#[derive(Debug, Clone)]
pub struct Solver1(pub Parsed1);

impl Solver for Solver1 {
    type Solution = anyhow::Result<Solution1>;

    fn solve(&self) -> Self::Solution {
        find_unique_window(self.0.iter(), 4).context("no start-of-packet marker detected")
    }
}

#[derive(Debug, Clone)]
pub struct Solver2(pub Parsed2);

impl Solver for Solver2 {
    type Solution = anyhow::Result<Solution2>;

    fn solve(&self) -> Self::Solution {
        find_unique_window(self.0.iter(), 14).context("no start-of-message marker detected")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Parser1, Parser2};
    use aoc::Parser;

    const INPUT: &'static str = include_str!("../input-test.txt");

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
            assert_eq!(Solver1(Parser1(input).parse()?).solve()?, expected);
        }
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        for (input, (_, expected)) in test_cases() {
            assert_eq!(Solver2(Parser2(input).parse()?).solve()?, expected);
        }
        Ok(())
    }
}
