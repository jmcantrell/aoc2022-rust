use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Range, Sub, SubAssign};

use num::{Num, PrimInt, Signed};

use super::{ColumnGroupIter, ColumnIter, ColumnLineIter, RowGroupIter, RowIter, RowLineIter};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Vector<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Num> Vector<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn transpose(self) -> Self {
        Self::new(self.y, self.x)
    }
}

impl<T: Signed> Vector<T> {
    pub fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    pub fn signum(self) -> Self {
        Self::new(self.x.signum(), self.y.signum())
    }
}

impl<T: PrimInt> Vector<T> {
    pub fn row_major_locations(yr: Range<T>, xr: Range<T>) -> RowIter<T> {
        RowIter::new(yr, xr)
    }

    pub fn column_major_locations(xr: Range<T>, yr: Range<T>) -> ColumnIter<T> {
        ColumnIter::new(xr, yr)
    }

    pub fn row_locations(row: T, xr: Range<T>) -> RowLineIter<T> {
        RowLineIter::new(row, xr)
    }

    pub fn column_locations(column: T, yr: Range<T>) -> ColumnLineIter<T> {
        ColumnLineIter::new(column, yr)
    }

    pub fn row_location_groups(yr: Range<T>, xr: Range<T>) -> RowGroupIter<T> {
        RowGroupIter::new(yr, xr)
    }

    pub fn column_location_groups(xr: Range<T>, yr: Range<T>) -> ColumnGroupIter<T> {
        ColumnGroupIter::new(xr, yr)
    }
}

impl<T: Num> From<(T, T)> for Vector<T> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl<T: Num> From<Vector<T>> for (T, T) {
    fn from(vector: Vector<T>) -> (T, T) {
        (vector.x, vector.y)
    }
}

impl<T: Num + fmt::Display> fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x {} and y {}", self.x, self.y)
    }
}

macro_rules! impl_op {
    ($trait:ident, $method:ident) => {
        impl<T: Num> $trait<Self> for Vector<T> {
            type Output = Self;

            fn $method(self, other: Self) -> Self::Output {
                Self::new(
                    $trait::$method(self.x, other.x),
                    $trait::$method(self.y, other.y),
                )
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
        impl<T: Num + $trait> $trait<Self> for Vector<T> {
            fn $method(&mut self, other: Self) {
                self.x.$method(other.x);
                self.y.$method(other.y);
            }
        }
    };
}

impl_op_assign!(AddAssign, add_assign);
impl_op_assign!(SubAssign, sub_assign);
impl_op_assign!(MulAssign, mul_assign);
impl_op_assign!(DivAssign, div_assign);
