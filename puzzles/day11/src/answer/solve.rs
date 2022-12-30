use crate::core::{Item, MonkeyTroop};

use super::{Parsed1, Parsed2};

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn monkey_business<F>(troop: &MonkeyTroop, rounds: usize, worry_reducer: F) -> usize
where
    F: Fn(Item) -> Item,
{
    let mut troop = troop.clone();

    for _ in 0..rounds {
        troop.iterate(&worry_reducer);
    }

    let mut inspections: Vec<_> = troop
        .monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect();

    inspections.sort();

    inspections.iter().rev().take(2).product()
}

pub fn solve1(troop: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(monkey_business(troop, 20, |item| item / 3))
}

pub fn solve2(troop: &Parsed2) -> anyhow::Result<Solution2> {
    let modulo: usize = troop
        .monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product();

    Ok(monkey_business(troop, 10_000, |item| item % modulo))
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 10_605);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 2_713_310_158);
        Ok(())
    }
}
