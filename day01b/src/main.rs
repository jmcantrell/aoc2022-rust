use std::convert::TryFrom;
use std::fs;

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

impl TryFrom<&str> for Elf {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            snacks: s
                .lines()
                .map(|line| line.parse::<Snack>())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut elves = input
        .split("\n\n")
        .map(|chunk| chunk.try_into())
        .collect::<Result<Vec<Elf>, _>>()?;

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

    Ok(())
}
