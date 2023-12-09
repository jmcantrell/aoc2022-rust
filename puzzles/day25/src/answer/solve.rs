use anyhow::Context;

use crate::core::SnafuNumber;

use super::Parsed;

pub type Solution = String;

pub fn solve(snafu_numbers: &Parsed) -> anyhow::Result<Solution> {
    let total = snafu_numbers
        .iter()
        .enumerate()
        .map(|(i, number)| {
            let n = number.to_decimal();
            usize::try_from(n).with_context(|| {
                format!(
                    "expected number {} to be positive, but it was: {}",
                    i + 1,
                    n
                )
            })
        })
        .sum::<Result<usize, _>>()?;

    Ok(SnafuNumber::from(total).to_string())
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::parse;

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_solve() -> anyhow::Result<()> {
        assert_eq!(solve(&parse(INPUT)?)?, "2=-1=0");
        Ok(())
    }
}
