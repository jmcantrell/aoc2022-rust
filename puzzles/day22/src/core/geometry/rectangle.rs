use std::ops::Range;

use super::{AxesBounds, Location, Size};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Rectangle {
    pub top_left: Location,
    pub bottom_right: Location,
}

impl Rectangle {
    pub const fn new(top_left: Location, bottom_right: Location) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }

    pub fn transpose(self) -> Self {
        Self {
            top_left: self.top_left.transpose(),
            bottom_right: self.bottom_right.transpose(),
        }
    }
}

impl AxesBounds for Rectangle {
    fn vertical_bounds(&self) -> Range<usize> {
        self.top_left.row..(self.bottom_right.row + 1)
    }

    fn horizontal_bounds(&self) -> Range<usize> {
        self.top_left.column..(self.bottom_right.column + 1)
    }

    fn top_left(&self) -> Location {
        self.top_left
    }

    fn bottom_right(&self) -> Location {
        self.bottom_right
    }

    fn bounds(&self) -> Self {
        self.clone()
    }
}

impl From<Size> for Rectangle {
    fn from(size: Size) -> Self {
        Self::new(Location::default(), Location::from(size) - Size::square(1))
    }
}

impl From<(Location, Size)> for Rectangle {
    fn from((top_left, size): (Location, Size)) -> Self {
        Self::new(top_left, top_left + size - Size::square(1))
    }
}

impl From<(Size, Location)> for Rectangle {
    fn from((size, bottom_right): (Size, Location)) -> Self {
        Self::new(bottom_right - size + Size::square(1), bottom_right)
    }
}

impl From<(Location, Location)> for Rectangle {
    fn from((top_left, bottom_right): (Location, Location)) -> Self {
        Self::new(top_left, bottom_right)
    }
}
