use crate::core::{Decryptor, Value};

use super::{Parsed, Parsed1, Parsed2};

pub type Solution = Value;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn decrypt(values: &Parsed, decryption_key: Value, iterations: usize) -> Solution {
    Decryptor::new(values.clone(), decryption_key)
        .decrypt(iterations)
        .into_iter()
        .sum()
}

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(decrypt(parsed, 1, 1))
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    Ok(decrypt(parsed, 811_589_153, 10))
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 3);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 1_623_178_306);
        Ok(())
    }
}
