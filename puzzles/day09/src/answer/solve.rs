use crate::core::Rope;

use super::Parsed;

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn trail_size(moves: &Parsed, num_knots: usize) -> usize {
    let mut rope = Rope::new(num_knots);

    for vector in moves.iter() {
        rope.drag(vector);
    }

    rope.trail.len()
}

pub fn solve1(vectors: &Parsed) -> anyhow::Result<Solution1> {
    Ok(trail_size(vectors, 2))
}

pub fn solve2(vectors: &Parsed) -> anyhow::Result<Solution2> {
    Ok(trail_size(vectors, 10))
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::parse;

    const INPUT1: Input = include_str!("../../input-test-1.txt");
    const INPUT2: Input = include_str!("../../input-test-2.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse(INPUT1)?)?, 13);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse(INPUT2)?)?, 36);
        Ok(())
    }
}
