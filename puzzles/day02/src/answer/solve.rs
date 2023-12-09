use crate::answer::{Parsed1, Parsed2};
use crate::core::{play, Round, Score};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn get_my_total_score(scores: impl Iterator<Item = Round>) -> Score {
    scores.map(|(_, (_, my_score))| my_score).sum()
}

pub fn solve1(pairs: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(get_my_total_score(
        pairs.iter().map(|(shape1, shape2)| play(shape1, shape2)),
    ))
}

pub fn solve2(pairs: &Parsed2) -> anyhow::Result<Solution2> {
    Ok(get_my_total_score(pairs.iter().map(
        |(shape, my_outcome)| play(shape, &my_outcome.ensure(shape)),
    )))
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 15);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 12);
        Ok(())
    }
}
