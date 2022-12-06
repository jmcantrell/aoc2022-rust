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

fn parse_elves(s: &str) -> anyhow::Result<Vec<Elf>> {
    Ok(s.split("\n\n")
        .map(|chunk| chunk.try_into())
        .collect::<Result<Vec<Elf>, _>>()?)
}

fn find_elf_with_most_calories(elves: &Vec<Elf>) -> Option<(usize, &Elf)> {
    elves
        .iter()
        .enumerate()
        .max_by_key(|(_, elf)| elf.total_calories())
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let elves = parse_elves(&input)?;
    let (index, elf) = find_elf_with_most_calories(&elves).context("no elves")?;

    println!(
        "Elf number {} is carrying the most calories ({}).",
        index + 1,
        elf.total_calories()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let elves = parse_elves(input)?;
        let (index, elf) = find_elf_with_most_calories(&elves).context("no elves")?;

        assert_eq!(index + 1, 4);
        assert_eq!(elf.total_calories(), 24000);

        Ok(())
    }
}
