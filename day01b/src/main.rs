use std::convert::TryFrom;
use std::fs;
use std::num::ParseIntError;

type Snack = usize;

#[derive(Debug)]
struct Elf {
    snacks: Vec<Snack>,
}

impl Elf {
    fn total_calories(&self) -> usize {
        self.snacks.iter().sum()
    }
}

#[derive(Debug)]
enum ParseElfError {
    InvalidCalories(ParseIntError),
}

impl From<ParseIntError> for ParseElfError {
    fn from(err: ParseIntError) -> Self {
        ParseElfError::InvalidCalories(err)
    }
}

impl TryFrom<&str> for Elf {
    type Error = ParseElfError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            snacks: s
                .lines()
                .map(|line| line.parse::<Snack>())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut elves = input
        .split("\n\n")
        .map(|chunk| chunk.try_into())
        .collect::<Result<Vec<Elf>, _>>()
        .unwrap();

    elves.sort_by_key(|elf| elf.total_calories());

    let count = 3;

    let total_calories: usize = elves
        .iter()
        .rev()
        .take(count)
        .map(|elf| elf.total_calories())
        .sum();

    println!(
        "The top {} elves with the most calories have {} calories",
        count, total_calories
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_calories_error() {
        let res = TryInto::<Elf>::try_into("bogus");
        assert!(matches!(res, Err(ParseElfError::InvalidCalories(_))));
    }
}
