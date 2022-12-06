use std::fs;

const NUM_ITEM_TYPES: usize = 26 * 2;
const GROUP_SIZE: usize = 3;

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

fn find_common_priority(rucksacks: &[&str]) -> anyhow::Result<usize> {
    let mut counts = [0; NUM_ITEM_TYPES];

    for &rucksack in rucksacks {
        let mut checklist = [false; NUM_ITEM_TYPES];

        for item in rucksack.chars() {
            let priority = get_item_priority(item)?;
            checklist[priority - 1] = true;
        }

        for i in 0..NUM_ITEM_TYPES {
            if checklist[i] {
                counts[i] += 1;
            }
        }
    }

    let n = rucksacks.len();

    for (i, &count) in counts.iter().enumerate() {
        if count >= n {
            return Ok(i + 1);
        }
    }

    unreachable!()
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let elves: Vec<_> = input.lines().collect();

    let priorities = elves
        .chunks(GROUP_SIZE)
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

        let elves: Vec<_> = input.lines().collect();

        let priorities = elves
            .chunks(GROUP_SIZE)
            .map(find_common_priority)
            .collect::<Result<Vec<_>, _>>()?;

        let sum_of_priorities: usize = priorities.iter().sum();

        assert_eq!(sum_of_priorities, 70);

        Ok(())
    }
}
