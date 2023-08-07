pub type Snack = usize;
pub type Elf = Vec<Snack>;
pub type Elves = Vec<Elf>;

pub fn sum_snacks_by_elf(elves: &Elves) -> impl Iterator<Item = Snack> + '_ {
    elves.iter().map(|elf| elf.iter().sum())
}
