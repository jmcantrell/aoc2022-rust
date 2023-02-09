use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Range, Sub, SubAssign};

use super::{AxesBounds, Location};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

impl Size {
    pub const fn new(height: usize, width: usize) -> Self {
        Self { height, width }
    }

    pub const fn square(size: usize) -> Self {
        Self {
            height: size,
            width: size,
        }
    }

    pub fn transpose(self) -> Self {
        Self {
            height: self.width,
            width: self.height,
        }
    }
}

impl AxesBounds for Size {
    fn vertical_bounds(&self) -> Range<usize> {
        0..self.height
    }

    fn horizontal_bounds(&self) -> Range<usize> {
        0..self.width
    }

    fn size(&self) -> Self {
        self.clone()
    }
}

impl From<Location> for Size {
    fn from(Location { row, column }: Location) -> Self {
        Self {
            height: row,
            width: column,
        }
    }
}

impl From<(&Location, &Location)> for Size {
    fn from((a, b): (&Location, &Location)) -> Self {
        Self {
            height: a.row.abs_diff(b.row) + 1,
            width: a.column.abs_diff(b.column) + 1,
        }
    }
}

macro_rules! impl_op {
    ($trait:ident, $method:ident) => {
        impl $trait<Self> for Size {
            type Output = Self;

            fn $method(self, other: Self) -> Self::Output {
                Self {
                    height: $trait::$method(self.height, other.height),
                    width: $trait::$method(self.width, other.width),
                }
            }
        }
    };
}

impl_op!(Add, add);
impl_op!(Sub, sub);
impl_op!(Mul, mul);
impl_op!(Div, div);

macro_rules! impl_op_assign {
    ($trait:ident, $method:ident) => {
        impl $trait<Self> for Size {
            fn $method(&mut self, other: Self) {
                self.height.$method(other.height);
                self.width.$method(other.width);
            }
        }
    };
}

impl_op_assign!(AddAssign, add_assign);
impl_op_assign!(SubAssign, sub_assign);
impl_op_assign!(MulAssign, mul_assign);
impl_op_assign!(DivAssign, div_assign);
