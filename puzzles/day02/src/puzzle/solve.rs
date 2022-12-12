use crate::core::{play, Round, Score};
use crate::puzzle::{Parsed1, Parsed2};

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn get_my_total_score(scores: impl Iterator<Item = Round>) -> Score {
    scores.map(|(_, (_, my_score))| my_score).sum()
}

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    let scores = parsed.iter().map(|(shape1, shape2)| play(shape1, shape2));

    Ok(get_my_total_score(scores))
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    let scores = parsed
        .iter()
        .map(|(shape, my_outcome)| play(shape, &my_outcome.ensure(shape)));

    Ok(get_my_total_score(scores))
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::puzzle::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 15);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 12);
        Ok(())
    }
}
