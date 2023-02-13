use Rock::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rock {
    Slab,
    Plus,
    Ell,
    Column,
    Square,
}

impl Rock {
    pub fn bytes(&self) -> [u8; 4] {
        match self {
            Slab => [0b00000000, 0b00000000, 0b00000000, 0b11110000],
            Plus => [0b00000000, 0b01000000, 0b11100000, 0b01000000],
            Ell => [0b00000000, 0b00100000, 0b00100000, 0b11100000],
            Column => [0b10000000, 0b10000000, 0b10000000, 0b10000000],
            Square => [0b00000000, 0b00000000, 0b11000000, 0b11000000],
        }
    }
}
