use std::ops::Range;

use num::PrimInt;

use super::Vector;

pub trait VectorIterator<T: PrimInt>:
    Clone + ExactSizeIterator + DoubleEndedIterator<Item = Vector<T>>
{
}

impl<T: PrimInt, I> VectorIterator<T> for I where
    I: Clone + ExactSizeIterator + DoubleEndedIterator<Item = Vector<T>>
{
}

pub trait VectorGroupIterator<T: PrimInt>:
    Clone + ExactSizeIterator + DoubleEndedIterator<Item = Self::VectorIter>
{
    type VectorIter: VectorIterator<T>;
}

impl<T: PrimInt, I, L> VectorGroupIterator<T> for I
where
    I: Clone + ExactSizeIterator + DoubleEndedIterator<Item = L>,
    L: VectorIterator<T>,
{
    type VectorIter = L;
}

macro_rules! impl_iter {
    ($item:ident, $p:ident, $s:ident) => {
        #[derive(Debug, Clone)]
        pub struct $item<T> {
            p_iter: Range<T>,
            p_curr: Option<T>,
            s_orig: Range<T>,
            s_iter: Range<T>,
        }

        impl<T: PrimInt> $item<T> {
            pub fn new(p_iter: Range<T>, s_iter: Range<T>) -> Self {
                Self {
                    p_iter,
                    p_curr: None,
                    s_orig: s_iter.clone(),
                    s_iter,
                }
            }
        }

        impl<T: PrimInt> Iterator for $item<T>
        where
            Range<T>: Iterator<Item = T>,
        {
            type Item = Vector<T>;

            fn next(&mut self) -> Option<Self::Item> {
                let s = match self.s_iter.next() {
                    None => {
                        self.p_curr = self.p_iter.next();
                        self.s_iter = self.s_orig.clone();
                        self.s_iter.next()
                    }
                    s => s,
                }?;

                let p = match self.p_curr {
                    None => {
                        self.p_curr = self.p_iter.next();
                        self.p_curr
                    }
                    p => p,
                }?;

                Some(Vector { $p: p, $s: s })
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                let (p_lower, maybe_p_upper) = self.p_iter.size_hint();
                let (s_lower, maybe_s_upper) = self.s_iter.size_hint();
                let (s0_lower, maybe_s0_upper) = self.s_orig.size_hint();

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

        impl<T: PrimInt> ExactSizeIterator for $item<T> where
            Range<T>: ExactSizeIterator + Iterator<Item = T>
        {
        }

        impl<T: PrimInt> DoubleEndedIterator for $item<T>
        where
            Range<T>: DoubleEndedIterator<Item = T>,
        {
            fn next_back(&mut self) -> Option<Self::Item> {
                let s = match self.s_iter.next_back() {
                    None => {
                        self.p_curr = self.p_iter.next_back();
                        self.s_iter = self.s_orig.clone();
                        self.s_iter.next_back()
                    }
                    s => s,
                }?;

                let p = match self.p_curr {
                    None => {
                        self.p_curr = self.p_iter.next_back();
                        self.p_curr
                    }
                    p => p,
                }?;

                Some(Vector { $p: p, $s: s })
            }
        }
    };
}

impl_iter!(RowIter, y, x);
impl_iter!(ColumnIter, x, y);

macro_rules! impl_line_iter {
    ($item:ident, $p:ident, $s:ident) => {
        #[derive(Debug, Clone)]
        pub struct $item<T> {
            p: T,
            s_iter: Range<T>,
        }

        impl<T> $item<T> {
            pub fn new(p: T, s_iter: Range<T>) -> Self {
                Self { p, s_iter }
            }
        }

        impl<T: PrimInt> Iterator for $item<T>
        where
            Range<T>: Iterator<Item = T>,
        {
            type Item = Vector<T>;

            fn next(&mut self) -> Option<Self::Item> {
                self.s_iter.next().map(|s| Vector { $p: self.p, $s: s })
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                self.s_iter.size_hint()
            }
        }

        impl<T: PrimInt> ExactSizeIterator for $item<T> where
            Range<T>: ExactSizeIterator + Iterator<Item = T>
        {
        }

        impl<T: PrimInt> DoubleEndedIterator for $item<T>
        where
            Range<T>: DoubleEndedIterator<Item = T>,
        {
            fn next_back(&mut self) -> Option<Self::Item> {
                self.s_iter
                    .next_back()
                    .map(|s| Vector { $p: self.p, $s: s })
            }
        }
    };
}

impl_line_iter!(RowLineIter, y, x);
impl_line_iter!(ColumnLineIter, x, y);

macro_rules! impl_group_iter {
    ($item:ident, $inner:ident, $p:ident, $s:ident) => {
        #[derive(Debug, Clone)]
        pub struct $item<T> {
            p_iter: Range<T>,
            s_iter: Range<T>,
        }

        impl<T> $item<T> {
            pub fn new(p_iter: Range<T>, s_iter: Range<T>) -> Self {
                Self { p_iter, s_iter }
            }
        }

        impl<T: Clone> Iterator for $item<T>
        where
            Range<T>: Iterator<Item = T>,
        {
            type Item = $inner<T>;

            fn next(&mut self) -> Option<Self::Item> {
                self.p_iter
                    .next()
                    .map(|p| Self::Item::new(p, self.s_iter.clone()))
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                self.p_iter.size_hint()
            }
        }

        impl<T: Clone> ExactSizeIterator for $item<T> where
            Range<T>: ExactSizeIterator + Iterator<Item = T>
        {
        }

        impl<T: Clone> DoubleEndedIterator for $item<T>
        where
            Range<T>: DoubleEndedIterator<Item = T>,
        {
            fn next_back(&mut self) -> Option<Self::Item> {
                self.p_iter
                    .next_back()
                    .map(|p| Self::Item::new(p, self.s_iter.clone()))
            }
        }
    };
}

impl_group_iter!(RowGroupIter, RowLineIter, y, x);
impl_group_iter!(ColumnGroupIter, ColumnLineIter, x, y);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_iter() {
        macro_rules! assert_range {
            ($yr:expr, $xr:expr, $expected:expr) => {
                assert_eq!(
                    RowIter::new($yr, $xr).collect::<Vec<_>>(),
                    Vec::from($expected.map(|pair| pair.into()))
                );
            };
        }

        assert_range!(
            1..4,
            2..5,
            [
                (2, 1),
                (3, 1),
                (4, 1),
                (2, 2),
                (3, 2),
                (4, 2),
                (2, 3),
                (3, 3),
                (4, 3),
            ]
        );
    }

    #[test]
    fn column_iter() {
        macro_rules! assert_range {
            ($xr:expr, $yr:expr, $expected:expr) => {
                assert_eq!(
                    ColumnIter::new($xr, $yr).collect::<Vec<_>>(),
                    Vec::from($expected.map(|pair| pair.into()))
                );
            };
        }

        assert_range!(
            1..4,
            2..5,
            [
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 2),
                (2, 3),
                (2, 4),
                (3, 2),
                (3, 3),
                (3, 4),
            ]
        );
    }

    #[test]
    fn row_line_iter() {
        macro_rules! assert_range {
            ($y:expr, $xr:expr, $expected:expr) => {
                assert_eq!(
                    RowLineIter::new($y, $xr).collect::<Vec<_>>(),
                    Vec::from($expected.map(|pair| pair.into()))
                );
            };
        }

        assert_range!(1, 2..6, [(2, 1), (3, 1), (4, 1), (5, 1),]);
    }

    #[test]
    fn column_line_iter() {
        macro_rules! assert_range {
            ($x:expr, $yr:expr, $expected:expr) => {
                assert_eq!(
                    ColumnLineIter::new($x, $yr).collect::<Vec<_>>(),
                    Vec::from($expected.map(|pair| pair.into()))
                );
            };
        }

        assert_range!(1, 2..6, [(1, 2), (1, 3), (1, 4), (1, 5),]);
    }

    #[test]
    fn row_group_iter() {
        macro_rules! assert_range {
            ($yr:expr, $xr:expr, $expected:expr) => {
                assert_eq!(
                    RowGroupIter::new($yr, $xr)
                        .map(|group| group.collect())
                        .collect::<Vec<Vec<_>>>(),
                    $expected
                        .into_iter()
                        .map(|group| group.into_iter().map(|pair| pair.into()).collect())
                        .collect::<Vec<Vec<_>>>()
                );
            };
        }

        assert_range!(
            1..4,
            2..5,
            [
                [(2, 1), (3, 1), (4, 1)],
                [(2, 2), (3, 2), (4, 2)],
                [(2, 3), (3, 3), (4, 3)],
            ]
        );
    }

    #[test]
    fn column_group_iter() {
        macro_rules! assert_range {
            ($xr:expr, $yr:expr, $expected:expr) => {
                assert_eq!(
                    ColumnGroupIter::new($xr, $yr)
                        .map(|group| group.collect())
                        .collect::<Vec<Vec<_>>>(),
                    $expected
                        .into_iter()
                        .map(|group| group.into_iter().map(|pair| pair.into()).collect())
                        .collect::<Vec<Vec<_>>>()
                );
            };
        }

        assert_range!(
            1..4,
            2..5,
            [
                [(1, 2), (1, 3), (1, 4)],
                [(2, 2), (2, 3), (2, 4)],
                [(3, 2), (3, 3), (3, 4)],
            ]
        );
    }
}
