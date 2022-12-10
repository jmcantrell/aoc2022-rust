use crate::parser::{Parsed1, Parsed2};
use anyhow::Context;
use aoc::Solver;

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

type Priority = usize;

const MAX_PRIORITY: Priority = 26 * 2;

fn find_common_priority(groups: &Vec<Vec<Priority>>) -> Option<Priority> {
    let mut counts = [0; MAX_PRIORITY];
    let mut num_groups = 0;

    for group in groups {
        num_groups += 1;

        let mut checklist = [false; MAX_PRIORITY];

        for priority in group {
            checklist[priority - 1] = true;
        }

        for i in 0..MAX_PRIORITY {
            if checklist[i] {
                counts[i] += 1;
            }
        }
    }

    for (i, &count) in counts.iter().enumerate() {
        if count >= num_groups {
            return Some(i + 1);
        }
    }

    None
}

#[derive(Debug, Clone)]
pub struct Solver1(pub Parsed1);

impl Solver for Solver1 {
    type Solution = anyhow::Result<Solution1>;

    fn solve(&self) -> Self::Solution {
        let common_priorities = self
            .0
            .iter()
            .enumerate()
            .map(|(i, rucksack)| {
                let (pocket1, pocket2) = rucksack.split_at(rucksack.len() / 2);
                let rucksack = vec![pocket1.to_vec(), pocket2.to_vec()];
                find_common_priority(&rucksack)
                    .with_context(|| format!("rucksack number {}", i + 1))
            })
            .collect::<Result<Vec<_>, _>>()
            .context("missing common priority")?;

        Ok(common_priorities.iter().sum())
    }
}

#[derive(Debug, Clone)]
pub struct Solver2(pub Parsed2);

impl Solver for Solver2 {
    type Solution = anyhow::Result<Priority>;

    fn solve(&self) -> Self::Solution {
        let common_priorities = self
            .0
            .chunks(3)
            .enumerate()
            .map(|(i, rucksacks)| {
                find_common_priority(&rucksacks.to_vec())
                    .with_context(|| format!("rucksack number {}", i + 1))
            })
            .collect::<Result<Vec<_>, _>>()
            .context("missing common priority")?;

        Ok(common_priorities.iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Parser1, Parser2};
    use aoc::Parser;

    const INPUT: &'static str = include_str!("../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(Solver1(Parser1(INPUT).parse()?).solve()?, 157);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(Solver2(Parser2(INPUT).parse()?).solve()?, 70);
        Ok(())
    }
}
