use std::fs;

const NUM_ITEM_TYPES: usize = 26 * 2;
const NUM_RUCKSACK_POCKETS: usize = 2;

fn get_item_priority(item: char) -> anyhow::Result<usize> {
    Ok(1 + (item as usize)
        - match item {
            'a'..='z' => 97,
            'A'..='Z' => 39,
            _ => {
                anyhow::bail!("Invalid item `{}`", item);
            }
        })
}

fn find_common_priority(rucksack: &str) -> anyhow::Result<usize> {
    let mut counts = [0; NUM_ITEM_TYPES];
    let pocket_size = rucksack.len() / NUM_RUCKSACK_POCKETS;

    for pocket in 0..NUM_RUCKSACK_POCKETS {
        let start = pocket * pocket_size;
        let end = start + pocket_size;

        let mut checklist = [false; NUM_ITEM_TYPES];

        for item in rucksack[start..end].chars() {
            let priority = get_item_priority(item)?;
            checklist[priority - 1] = true;
        }

        for i in 0..NUM_ITEM_TYPES {
            if checklist[i] {
                counts[i] += 1;
            }
        }
    }

    for i in 0..NUM_ITEM_TYPES {
        if counts[i] >= NUM_RUCKSACK_POCKETS {
            return Ok(i + 1);
        }
    }

    unreachable!()
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let priorities = input
        .lines()
        .map(find_common_priority)
        .collect::<Result<Vec<_>, _>>()?;

    let sum_of_priorities: usize = priorities.iter().sum();

    dbg!(sum_of_priorities);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let input = concat!(
            "vJrwpWtwJgWrhcsFMMfFFhFp\n",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n",
            "PmmdzqPrVvPwwTWBwg\n",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n",
            "ttgJtRGJQctTZtZT\n",
            "CrZsJsPPZsGzwwsLwLmpwMDw\n",
        );

        let priorities = input
            .lines()
            .map(find_common_priority)
            .collect::<Result<Vec<_>, _>>()?;

        let sum_of_priorities: usize = priorities.iter().sum();

        assert_eq!(sum_of_priorities, 157);

        Ok(())
    }
}
