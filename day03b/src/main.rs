use std::fs;

const NUM_ITEM_TYPES: usize = 26 * 2;
const GROUP_SIZE: usize = 3;

fn get_item_priority(item: char) -> usize {
    1 + (item as usize)
        - match item {
            'a'..='z' => 97,
            'A'..='Z' => 39,
            _ => panic!("invalid character"),
        }
}

fn find_common_priority(rucksacks: &[&str]) -> usize {
    let mut counts = [0; NUM_ITEM_TYPES];

    for &rucksack in rucksacks {
        let mut checklist = [false; NUM_ITEM_TYPES];

        for item in rucksack.chars() {
            checklist[get_item_priority(item) - 1] = true;
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
            return i + 1;
        }
    }

    unreachable!()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let elves: Vec<_> = input.lines().collect();

    let sum_of_priorities: usize = elves.chunks(GROUP_SIZE).map(find_common_priority).sum();

    dbg!(sum_of_priorities);
}
