use anyhow::Context;

use super::Parsed;

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(map: &Parsed) -> anyhow::Result<Solution1> {
    Ok(map.find_shortest_path().context("no path found")?.len())
}

pub fn solve2(map: &Parsed) -> anyhow::Result<Solution2> {
    map.find_alternate_paths()
        .map(|path| path.len())
        .min()
        .context("no paths found")
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::parse;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse(INPUT)?)?, 31);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse(INPUT)?)?, 29);
        Ok(())
    }
}
