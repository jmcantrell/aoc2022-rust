use anyhow::Context;

pub type Priority = usize;
pub type Rucksack = Vec<Priority>;
pub type Rucksacks = Vec<Rucksack>;

const MAX_PRIORITY: Priority = 26 * 2;

pub fn find_common_in_groups(groups: &[Rucksack]) -> Option<Priority> {
    let mut counts = [0; MAX_PRIORITY];
    let mut num_groups = 0;

    for group in groups {
        num_groups += 1;

        let mut checklist = [false; MAX_PRIORITY];

        for priority in group {
            checklist[priority - 1] = true;
        }

        for i in 0..MAX_PRIORITY {
            if checklist[i] {
                counts[i] += 1;
            }
        }
    }

    for (i, &count) in counts.iter().enumerate() {
        if count >= num_groups {
            return Some(i + 1);
        }
    }

    None
}

pub fn find_common_by_pocket(rucksacks: &[Rucksack]) -> anyhow::Result<Vec<Priority>> {
    rucksacks
        .iter()
        .enumerate()
        .map(|(i, rucksack)| {
            let (pocket1, pocket2) = rucksack.split_at(rucksack.len() / 2);
            find_common_in_groups(&[pocket1.to_vec(), pocket2.to_vec()])
                .with_context(|| format!("rucksack number {}", i + 1))
        })
        .collect()
}

pub fn find_common_by_chunk(rucksacks: &Rucksacks, n: usize) -> anyhow::Result<Vec<Priority>> {
    rucksacks
        .chunks(n)
        .enumerate()
        .map(|(i, rucksacks)| {
            find_common_in_groups(rucksacks).with_context(|| format!("chunk number {}", i + 1))
        })
        .collect()
}
