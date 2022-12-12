pub type Snack = usize;
pub type Elf = Vec<Snack>;
pub type Elves = Vec<Elf>;

pub fn sum_snacks_by_elf(elves: &Elves) -> Vec<Snack> {
    elves.iter().map(|elf| elf.iter().sum()).collect()
}
