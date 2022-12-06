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

fn find_elves_with_most_calories(elves: &Vec<Elf>, n: usize) -> Vec<(usize, &Elf)> {
    let mut elves: Vec<(usize, _)> = elves.iter().enumerate().collect();
    elves.sort_by_key(|(_, elf)| elf.total_calories());
    elves.reverse();
    elves.into_iter().take(n).collect()
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let elves = parse_elves(&input)?;

    let num = 3;
    let top_elves = find_elves_with_most_calories(&elves, num);
    let total_calories: usize = top_elves.iter().map(|(_, elf)| elf.total_calories()).sum();

    println!("The {} elves with the most calories:", num);

    for (index, elf) in top_elves.iter() {
        println!(
            "Elf number {} is carrying {} calories.",
            index + 1,
            elf.total_calories()
        );
    }

    println!("The total calories carried by them is {}.", total_calories);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let elves = parse_elves(input)?;

        let num = 3;
        let top_elves = find_elves_with_most_calories(&elves, num);
        let total_calories: usize = top_elves.iter().map(|(_, elf)| elf.total_calories()).sum();

        assert_eq!(
            top_elves
                .iter()
                .map(|(i, elf)| (*i, elf.total_calories()))
                .collect::<Vec<_>>(),
            vec![(3, 24000), (2, 11000), (4, 10000)]
        );

        assert_eq!(total_calories, 45000);
        Ok(())
    }
}
