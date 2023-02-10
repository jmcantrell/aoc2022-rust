use std::ops::Range;

use num::{zero, PrimInt, Unsigned};

use super::{AxesBounds, Vector};

pub type Size<T> = Vector<T>;

impl<T: PrimInt + Unsigned> Size<T> {
    pub const fn square(size: T) -> Self {
        Self::new(size, size)
    }

    pub fn area(&self) -> T {
        self.x * self.y
    }
}

impl<T: PrimInt + Unsigned> AxesBounds<T> for Size<T> {
    fn vertical_bounds(&self) -> Range<T> {
        zero()..self.y
    }

    fn horizontal_bounds(&self) -> Range<T> {
        zero()..self.x
    }
}
