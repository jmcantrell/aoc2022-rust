use anyhow::Context;
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

    let elves = input
        .split("\n\n")
        .map(|chunk| chunk.try_into())
        .collect::<Result<Vec<Elf>, _>>()?;

    let elf_with_all_the_snacks = elves
        .iter()
        .max_by_key(|elf| elf.total_calories())
        .context("No elves")?;

    println!(
        "The elf with the most calories has {} calories",
        elf_with_all_the_snacks.total_calories()
    );

    Ok(())
}
