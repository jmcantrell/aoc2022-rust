use std::ops::Range;

use num::{one, PrimInt};

use super::{
    CardinalDirection, ColumnGroupIter, ColumnIter, ColumnLineIter, Location, Rectangle,
    RowGroupIter, RowIter, RowLineIter, Size,
};

pub trait AxesBounds<T: PrimInt> {
    fn vertical_bounds(&self) -> Range<T>;

    fn horizontal_bounds(&self) -> Range<T>;

    fn top_left(&self) -> Location<T> {
        Location::new(self.top(), self.left())
    }

    fn top_right(&self) -> Location<T> {
        Location::new(self.top(), self.right())
    }

    fn bottom_left(&self) -> Location<T> {
        Location::new(self.bottom(), self.left())
    }

    fn bottom_right(&self) -> Location<T> {
        Location::new(self.bottom(), self.right())
    }

    fn bounds(&self) -> Rectangle<T> {
        Rectangle::new(self.top_left(), self.bottom_right())
    }

    fn height(&self) -> T {
        self.vertical_bounds().end - self.vertical_bounds().start
    }

    fn width(&self) -> T {
        self.horizontal_bounds().end - self.horizontal_bounds().start
    }

    fn top(&self) -> T {
        self.vertical_bounds().start
    }

    fn bottom(&self) -> T {
        self.vertical_bounds().end - one()
    }

    fn left(&self) -> T {
        self.horizontal_bounds().start
    }

    fn right(&self) -> T {
        self.horizontal_bounds().end - one()
    }

    fn size(&self) -> Size<T> {
        Size::new(self.width(), self.height())
    }

    fn area(&self) -> T {
        self.height() * self.width()
    }

    fn contains(&self, location: &Location<T>) -> bool {
        self.vertical_bounds().contains(&location.y)
            && self.horizontal_bounds().contains(&location.x)
    }

    fn neighbor(
        &self,
        location: &Location<T>,
        direction: &CardinalDirection,
    ) -> Option<Location<T>> {
        location
            .neighbor(direction)
            .and_then(|adjacent| self.contains(&adjacent).then_some(adjacent))
    }

    fn row_major_locations(&self) -> RowIter<T> {
        RowIter::new(self.vertical_bounds(), self.horizontal_bounds())
    }

    fn column_major_locations(&self) -> ColumnIter<T> {
        ColumnIter::new(self.horizontal_bounds(), self.vertical_bounds())
    }

    fn row_groups(&self) -> RowGroupIter<T> {
        RowGroupIter::new(self.vertical_bounds(), self.horizontal_bounds())
    }

    fn column_groups(&self) -> ColumnGroupIter<T> {
        ColumnGroupIter::new(self.horizontal_bounds(), self.vertical_bounds())
    }

    fn row_locations(&self, row: T) -> RowLineIter<T> {
        RowLineIter::new(row, self.horizontal_bounds())
    }

    fn column_locations(&self, column: T) -> ColumnLineIter<T> {
        ColumnLineIter::new(column, self.vertical_bounds())
    }

    fn first_row_locations(&self) -> RowLineIter<T> {
        RowLineIter::new(self.top(), self.horizontal_bounds())
    }

    fn last_row_locations(&self) -> RowLineIter<T> {
        RowLineIter::new(self.bottom(), self.horizontal_bounds())
    }

    fn first_column_locations(&self) -> ColumnLineIter<T> {
        ColumnLineIter::new(self.left(), self.vertical_bounds())
    }

    fn last_column_locations(&self) -> ColumnLineIter<T> {
        ColumnLineIter::new(self.right(), self.vertical_bounds())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Foo;

    impl AxesBounds<u8> for Foo {
        fn vertical_bounds(&self) -> Range<u8> {
            1..4
        }

        fn horizontal_bounds(&self) -> Range<u8> {
            2..4
        }
    }

    #[test]
    fn contains() {
        let foo = Foo;

        for location in foo.row_major_locations() {
            assert!(foo.contains(&location))
        }

        use CardinalDirection::*;

        macro_rules! assert_edge {
            ($edge:expr, $iter:expr) => {
                for location in $iter {
                    let outside = location.neighbor(&$edge).unwrap();
                    assert!(!foo.contains(&outside));
                }
            };
        }

        assert_edge!(North, foo.first_row_locations());
        assert_edge!(South, foo.last_row_locations());
        assert_edge!(West, foo.first_column_locations());
        assert_edge!(East, foo.last_column_locations());
    }

    #[test]
    fn neighbor() {
        let foo = Foo;

        macro_rules! assert_neighbor {
            ($location:expr, $direction:expr, $adjacent:expr) => {
                assert_eq!(foo.neighbor(&$location, &$direction), $adjacent);
            };
        }

        macro_rules! assert_edge {
            ($edge:expr, $iter:expr) => {
                for location in $iter {
                    assert_neighbor!(location, &$edge, None);
                    let outside = location.neighbor(&$edge).unwrap();
                    assert_neighbor!(outside, $edge.inverse(), Some(location));
                }
            };
        }

        use CardinalDirection::*;

        assert_edge!(North, foo.first_row_locations());
        assert_edge!(South, foo.last_row_locations());
        assert_edge!(West, foo.first_column_locations());
        assert_edge!(East, foo.last_column_locations());
    }
}
