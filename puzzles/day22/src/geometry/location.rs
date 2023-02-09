use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::{Direction, Size};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Location {
    pub row: usize,
    pub column: usize,
}

impl Location {
    pub const fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub fn transpose(self) -> Self {
        Self {
            row: self.column,
            column: self.row,
        }
    }

    pub fn row_range<C>(row: usize, column_bounds: C) -> RowLineIter<C> {
        RowLineIter::new(row, column_bounds)
    }

    pub fn column_range<R>(column: usize, row_bounds: R) -> ColumnLineIter<R> {
        ColumnLineIter::new(column, row_bounds)
    }

    pub fn rows<R, C>(row_bounds: R, column_bounds: C) -> RowGroupIter<R, C> {
        RowGroupIter::new(row_bounds, column_bounds)
    }

    pub fn columns<C, R>(column_bounds: C, row_bounds: R) -> ColumnGroupIter<C, R> {
        ColumnGroupIter::new(column_bounds, row_bounds)
    }

    pub fn neighbor(self, direction: &Direction) -> Option<Self> {
        use Direction::*;
        match direction {
            North => self.row.checked_sub(1).map(|row| Location {
                row,
                column: self.column,
            }),
            South => self.row.checked_add(1).map(|row| Location {
                row,
                column: self.column,
            }),
            West => self.column.checked_sub(1).map(|column| Location {
                column,
                row: self.row,
            }),
            East => self.column.checked_add(1).map(|column| Location {
                column,
                row: self.row,
            }),
        }
    }
}

impl From<(usize, usize)> for Location {
    fn from((row, column): (usize, usize)) -> Self {
        Location::new(row, column)
    }
}

impl From<Location> for (usize, usize) {
    fn from(location: Location) -> (usize, usize) {
        (location.row, location.column)
    }
}

impl From<Size> for Location {
    fn from(Size { height, width }: Size) -> Self {
        Self {
            row: height,
            column: width,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "row {} and column {}", self.row, self.column)
    }
}

impl Sub<Self> for Location {
    type Output = Size;

    fn sub(self, other: Self) -> Self::Output {
        Size {
            height: self.row - other.row,
            width: self.column - other.column,
        }
    }
}

macro_rules! impl_size_op {
    ($trait:ident, $method:ident) => {
        impl $trait<Size> for Location {
            type Output = Self;

            fn $method(self, size: Size) -> Self::Output {
                Self {
                    row: $trait::$method(self.row, size.height),
                    column: $trait::$method(self.column, size.width),
                }
            }
        }
    };
}

impl_size_op!(Add, add);
impl_size_op!(Sub, sub);
impl_size_op!(Mul, mul);
impl_size_op!(Div, div);

macro_rules! impl_size_int_assign {
    ($trait:ident, $method:ident) => {
        impl $trait<Size> for Location {
            fn $method(&mut self, size: Size) {
                self.row.$method(size.height);
                self.column.$method(size.width);
            }
        }
    };
}

impl_size_int_assign!(AddAssign, add_assign);
impl_size_int_assign!(SubAssign, sub_assign);
impl_size_int_assign!(MulAssign, mul_assign);
impl_size_int_assign!(DivAssign, div_assign);

pub trait LocationIterator: ExactSizeIterator + DoubleEndedIterator<Item = Location> {}

impl<I> LocationIterator for I where I: ExactSizeIterator + DoubleEndedIterator<Item = Location> {}

pub trait LocationGroupIterator:
    ExactSizeIterator + DoubleEndedIterator<Item = Self::LocationIter>
{
    type LocationIter: LocationIterator;
}

impl<I, L> LocationGroupIterator for I
where
    I: ExactSizeIterator + DoubleEndedIterator<Item = L>,
    L: LocationIterator,
{
    type LocationIter = L;
}

macro_rules! impl_iter {
    ($item:ident, $primary:ident, $secondary:ident) => {
        #[derive(Debug)]
        pub struct $item<P, S> {
            primary_iter: P,
            primary_current: Option<usize>,
            secondary_orig: S,
            secondary_iter: S,
        }

        impl<P, S> $item<P, S>
        where
            S: Iterator<Item = usize> + Clone,
        {
            pub fn new(primary_iter: P, secondary_iter: S) -> Self {
                Self {
                    primary_iter,
                    primary_current: None,
                    secondary_orig: secondary_iter.clone(),
                    secondary_iter,
                }
            }
        }

        impl<P, S> Iterator for $item<P, S>
        where
            P: Iterator<Item = usize>,
            S: Iterator<Item = usize> + Clone,
        {
            type Item = Location;

            fn next(&mut self) -> Option<Self::Item> {
                let secondary = match self.secondary_iter.next() {
                    None => {
                        self.primary_current = self.primary_iter.next();
                        self.secondary_iter = self.secondary_orig.clone();
                        self.secondary_iter.next()
                    }
                    secondary => secondary,
                }?;

                let primary = match self.primary_current {
                    None => {
                        self.primary_current = self.primary_iter.next();
                        self.primary_current
                    }
                    primary => primary,
                }?;

                Some(Location {
                    $primary: primary,
                    $secondary: secondary,
                })
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                let (p_lower, maybe_p_upper) = self.primary_iter.size_hint();
                let (s_lower, maybe_s_upper) = self.secondary_iter.size_hint();
                let (s0_lower, maybe_s0_upper) = self.secondary_orig.size_hint();

                let lower = p_lower.saturating_mul(s0_lower).saturating_add(s_lower);

                let maybe_upper = maybe_p_upper
                    .zip(maybe_s0_upper)
                    .map(|(p_upper, s0_upper)| {
                        p_upper
                            .checked_mul(s0_upper)
                            .zip(maybe_s_upper)
                            .map(|(ps0, s)| ps0.checked_add(s))
                            .flatten()
                    })
                    .flatten();

                (lower, maybe_upper)
            }
        }

        impl<P, S> ExactSizeIterator for $item<P, S>
        where
            P: ExactSizeIterator + Iterator<Item = usize>,
            S: ExactSizeIterator + Iterator<Item = usize> + Clone,
        {
        }

        impl<P, S> DoubleEndedIterator for $item<P, S>
        where
            P: DoubleEndedIterator<Item = usize>,
            S: DoubleEndedIterator<Item = usize> + Clone,
        {
            fn next_back(&mut self) -> Option<Self::Item> {
                let secondary = match self.secondary_iter.next_back() {
                    None => {
                        self.primary_current = self.primary_iter.next_back();
                        self.secondary_iter = self.secondary_orig.clone();
                        self.secondary_iter.next_back()
                    }
                    secondary => secondary,
                }?;

                let primary = match self.primary_current {
                    None => {
                        self.primary_current = self.primary_iter.next_back();
                        self.primary_current
                    }
                    primary => primary,
                }?;

                Some(Location {
                    $primary: primary,
                    $secondary: secondary,
                })
            }
        }
    };
}

impl_iter!(RowIter, row, column);
impl_iter!(ColumnIter, column, row);

macro_rules! impl_line_iter {
    ($item:ident, $primary:ident, $secondary:ident) => {
        #[derive(Debug)]
        pub struct $item<I> {
            primary: usize,
            secondary_iter: I,
        }

        impl<I> $item<I> {
            pub fn new(primary: usize, secondary_iter: I) -> Self {
                Self {
                    primary,
                    secondary_iter,
                }
            }
        }

        impl<I> Iterator for $item<I>
        where
            I: Iterator<Item = usize>,
        {
            type Item = Location;

            fn next(&mut self) -> Option<Self::Item> {
                self.secondary_iter.next().map(|secondary| Location {
                    $primary: self.primary,
                    $secondary: secondary,
                })
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                self.secondary_iter.size_hint()
            }
        }

        impl<I> ExactSizeIterator for $item<I> where I: ExactSizeIterator + Iterator<Item = usize> {}

        impl<I> DoubleEndedIterator for $item<I>
        where
            I: DoubleEndedIterator<Item = usize>,
        {
            fn next_back(&mut self) -> Option<Self::Item> {
                self.secondary_iter.next_back().map(|secondary| Location {
                    $primary: self.primary,
                    $secondary: secondary,
                })
            }
        }
    };
}

impl_line_iter!(RowLineIter, row, column);
impl_line_iter!(ColumnLineIter, column, row);

macro_rules! impl_group_iter {
    ($item:ident, $inner_item:ident) => {
        #[derive(Debug)]
        pub struct $item<P, S> {
            primary_iter: P,
            secondary_iter: S,
        }

        impl<P, S> $item<P, S> {
            pub fn new(primary_iter: P, secondary_iter: S) -> Self {
                Self {
                    primary_iter,
                    secondary_iter,
                }
            }
        }

        impl<P, S> Iterator for $item<P, S>
        where
            P: Iterator<Item = usize>,
            S: Iterator<Item = usize> + Clone,
        {
            type Item = $inner_item<S>;

            fn next(&mut self) -> Option<Self::Item> {
                self.primary_iter
                    .next()
                    .map(|primary| $inner_item::new(primary, self.secondary_iter.clone()))
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                self.primary_iter.size_hint()
            }
        }

        impl<P, S> ExactSizeIterator for $item<P, S>
        where
            P: ExactSizeIterator + Iterator<Item = usize>,
            S: ExactSizeIterator + Iterator<Item = usize> + Clone,
        {
        }

        impl<P, S> DoubleEndedIterator for $item<P, S>
        where
            P: DoubleEndedIterator<Item = usize>,
            S: DoubleEndedIterator<Item = usize> + Clone,
        {
            fn next_back(&mut self) -> Option<Self::Item> {
                self.primary_iter
                    .next_back()
                    .map(|primary| $inner_item::new(primary, self.secondary_iter.clone()))
            }
        }
    };
}

impl_group_iter!(RowGroupIter, RowLineIter);
impl_group_iter!(ColumnGroupIter, ColumnLineIter);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_iter() {
        assert_eq!(
            RowIter::new(1..=3, 2..5).collect::<Vec<_>>(),
            vec![
                Location { row: 1, column: 2 },
                Location { row: 1, column: 3 },
                Location { row: 1, column: 4 },
                Location { row: 2, column: 2 },
                Location { row: 2, column: 3 },
                Location { row: 2, column: 4 },
                Location { row: 3, column: 2 },
                Location { row: 3, column: 3 },
                Location { row: 3, column: 4 },
            ]
        );
    }

    #[test]
    fn column_iter() {
        assert_eq!(
            ColumnIter::new(1..=3, 2..5).collect::<Vec<_>>(),
            vec![
                Location { column: 1, row: 2 },
                Location { column: 1, row: 3 },
                Location { column: 1, row: 4 },
                Location { column: 2, row: 2 },
                Location { column: 2, row: 3 },
                Location { column: 2, row: 4 },
                Location { column: 3, row: 2 },
                Location { column: 3, row: 3 },
                Location { column: 3, row: 4 },
            ]
        );
    }

    #[test]
    fn row_line_iter() {
        assert_eq!(
            RowLineIter::new(1, 2..=5).collect::<Vec<_>>(),
            vec![
                Location { row: 1, column: 2 },
                Location { row: 1, column: 3 },
                Location { row: 1, column: 4 },
                Location { row: 1, column: 5 },
            ]
        );
    }

    #[test]
    fn column_line_iter() {
        assert_eq!(
            ColumnLineIter::new(1, 2..=5).collect::<Vec<_>>(),
            vec![
                Location { column: 1, row: 2 },
                Location { column: 1, row: 3 },
                Location { column: 1, row: 4 },
                Location { column: 1, row: 5 },
            ]
        );
    }

    #[test]
    fn row_group_iter() {
        macro_rules! assert_groups {
            ($primary:expr, $secondary:expr, $expected:expr) => {
                assert_eq!(
                    RowGroupIter::new($primary, $secondary)
                        .map(|row| row.collect())
                        .collect::<Vec<Vec<_>>>(),
                    $expected
                        .into_iter()
                        .map(|group| group
                            .into_iter()
                            .map(|(row, column)| Location { row, column })
                            .collect())
                        .collect::<Vec<Vec<_>>>()
                );
            };
        }

        assert_groups!(
            1..=3,
            2..=4,
            vec![
                vec![(1, 2), (1, 3), (1, 4)],
                vec![(2, 2), (2, 3), (2, 4)],
                vec![(3, 2), (3, 3), (3, 4)],
            ]
        );
    }

    #[test]
    fn column_group_iter() {
        macro_rules! assert_groups {
            ($primary:expr, $secondary:expr, $expected:expr) => {
                assert_eq!(
                    ColumnGroupIter::new($primary, $secondary)
                        .map(|column| column.collect())
                        .collect::<Vec<Vec<_>>>(),
                    $expected
                        .into_iter()
                        .map(|group| group
                            .into_iter()
                            .map(|(column, row)| Location { column, row })
                            .collect())
                        .collect::<Vec<Vec<_>>>()
                );
            };
        }

        assert_groups!(
            1..=3,
            2..=4,
            vec![
                vec![(1, 2), (1, 3), (1, 4)],
                vec![(2, 2), (2, 3), (2, 4)],
                vec![(3, 2), (3, 3), (3, 4)],
            ]
        );
    }
}
