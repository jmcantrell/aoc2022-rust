use crate::rps::{Outcome, Round, Score, Shape};
use aoc::Solver;

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn score_play(shape1: &Shape, shape2: &Shape) -> Round {
    let score1 = shape1.score();
    let score2 = shape2.score();

    match shape1.against(shape2) {
        Outcome::Win => (score1 + 6, score2),
        Outcome::Lose => (score1, score2 + 6),
        Outcome::Draw => (score1 + 3, score2 + 3),
    }
}

fn get_my_total_score(scores: impl Iterator<Item = Round>) -> Score {
    scores.map(|(_, my_score)| my_score).sum()
}

#[derive(Debug, Clone)]
pub struct Solver1(pub Vec<(Shape, Shape)>);

impl Solver for Solver1 {
    type Solution = anyhow::Result<Solution1>;

    fn solve(&self) -> Self::Solution {
        let scores = self
            .0
            .iter()
            .map(|(shape1, shape2)| score_play(shape1, shape2));

        Ok(get_my_total_score(scores))
    }
}

#[derive(Debug, Clone)]
pub struct Solver2(pub Vec<(Shape, Outcome)>);

impl Solver for Solver2 {
    type Solution = anyhow::Result<Solution2>;

    fn solve(&self) -> Self::Solution {
        let scores = self
            .0
            .iter()
            .map(|(shape, my_outcome)| score_play(shape, &my_outcome.ensure(shape)));

        Ok(get_my_total_score(scores))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Parser1, Parser2};
    use aoc::Parser;

    const INPUT: &'static str = include_str!("../input-test.txt");

    #[test]
    fn part1() -> anyhow::Result<()> {
        assert_eq!(Solver1(Parser1(INPUT).parse()?).solve()?, 15);
        Ok(())
    }

    #[test]
    fn part2() -> anyhow::Result<()> {
        assert_eq!(Solver2(Parser2(INPUT).parse()?).solve()?, 12);
        Ok(())
    }
}
