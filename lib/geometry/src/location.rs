use super::{CardinalDirection, Vector};

use num::{one, CheckedAdd, CheckedSub, Num, One};

pub type Location<T> = Vector<T>;

impl<T: Num + CheckedSub + CheckedAdd + One> Location<T> {
    pub fn neighbor(self, direction: &CardinalDirection) -> Option<Self> {
        use CardinalDirection::*;

        let (x, y) = self.into();

        match direction {
            West => x.checked_sub(&one()).map(|x| Self::new(x, y)),
            East => x.checked_add(&one()).map(|x| Self::new(x, y)),
            North => y.checked_sub(&one()).map(|y| Self::new(x, y)),
            South => y.checked_add(&one()).map(|y| Self::new(x, y)),
        }
    }
}
