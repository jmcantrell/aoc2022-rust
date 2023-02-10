use std::ops::Range;

use num::{one, Num, PrimInt};

use super::{AxesBounds, Location};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Rectangle<T: Num> {
    pub top_left: Location<T>,
    pub bottom_right: Location<T>,
}

impl<T: Num> Rectangle<T> {
    pub const fn new(top_left: Location<T>, bottom_right: Location<T>) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }
}

impl<T: PrimInt> AxesBounds<T> for Rectangle<T> {
    fn vertical_bounds(&self) -> Range<T> {
        self.top_left.y..(self.bottom_right.y + one())
    }

    fn horizontal_bounds(&self) -> Range<T> {
        self.top_left.x..(self.bottom_right.x + one())
    }
}
