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
            Self::Slab => [0b_00000000, 0b_00000000, 0b_00000000, 0b_11110000],
            Self::Plus => [0b_00000000, 0b_01000000, 0b_11100000, 0b_01000000],
            Self::Ell => [0b_00000000, 0b_00100000, 0b_00100000, 0b_11100000],
            Self::Column => [0b_10000000, 0b_10000000, 0b_10000000, 0b_10000000],
            Self::Square => [0b_00000000, 0b_00000000, 0b_11000000, 0b_11000000],
        }
    }
}
