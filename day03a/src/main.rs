use std::fs;

const NUM_ITEM_TYPES: usize = 26 * 2;
const NUM_RUCKSACK_POCKETS: usize = 2;

fn get_item_priority(item: char) -> usize {
    1 + (item as usize)
        - match item {
            'a'..='z' => 97,
            'A'..='Z' => 39,
            _ => panic!("invalid character"),
        }
}

fn find_common_priority(rucksack: &str) -> usize {
    let mut counts = [0; NUM_ITEM_TYPES];
    let pocket_size = rucksack.len() / NUM_RUCKSACK_POCKETS;

    for pocket in 0..NUM_RUCKSACK_POCKETS {
        let start = pocket * pocket_size;
        let end = start + pocket_size;

        let mut checklist = [false; NUM_ITEM_TYPES];

        for item in rucksack[start..end].chars() {
            checklist[get_item_priority(item) - 1] = true;
        }

        for i in 0..NUM_ITEM_TYPES {
            if checklist[i] {
                counts[i] += 1;
            }
        }
    }

    for i in 0..NUM_ITEM_TYPES {
        if counts[i] >= NUM_RUCKSACK_POCKETS {
            return i + 1;
        }
    }

    unreachable!()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let sum_of_priorities: usize = input.lines().map(find_common_priority).sum();

    dbg!(sum_of_priorities);
}
