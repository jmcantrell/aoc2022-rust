use std::ops::Range;

use super::{
    ColumnGroupIter, ColumnIter, Direction, Location, Rectangle, RowGroupIter, RowIter, Size,
};

pub trait AxesBounds {
    fn vertical_bounds(&self) -> Range<usize>;

    fn horizontal_bounds(&self) -> Range<usize>;

    fn top_left(&self) -> Location {
        Location::new(self.top(), self.left())
    }

    fn top_right(&self) -> Location {
        Location::new(self.top(), self.right())
    }

    fn bottom_left(&self) -> Location {
        Location::new(self.bottom(), self.left())
    }

    fn bottom_right(&self) -> Location {
        Location::new(self.bottom(), self.right())
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(self.top_left(), self.bottom_right())
    }

    fn height(&self) -> usize {
        self.vertical_bounds().end - self.vertical_bounds().start
    }

    fn width(&self) -> usize {
        self.horizontal_bounds().end - self.horizontal_bounds().start
    }

    fn top(&self) -> usize {
        self.vertical_bounds().start
    }

    fn bottom(&self) -> usize {
        self.vertical_bounds().end - 1
    }

    fn left(&self) -> usize {
        self.horizontal_bounds().start
    }

    fn right(&self) -> usize {
        self.horizontal_bounds().end - 1
    }

    fn size(&self) -> Size {
        Size::new(self.height(), self.width())
    }

    fn area(&self) -> usize {
        self.height() * self.width()
    }

    fn contains(&self, location: &Location) -> bool {
        self.vertical_bounds().contains(&location.row)
            && self.horizontal_bounds().contains(&location.column)
    }

    fn neighbor(&self, location: &Location, direction: &Direction) -> Option<Location> {
        location
            .neighbor(direction)
            .and_then(|adjacent| self.contains(&adjacent).then_some(adjacent))
    }

    fn locations_by_row(&self) -> RowIter<Range<usize>, Range<usize>> {
        RowIter::new(self.vertical_bounds(), self.horizontal_bounds())
    }

    fn locations_by_column(&self) -> ColumnIter<Range<usize>, Range<usize>> {
        ColumnIter::new(self.horizontal_bounds(), self.vertical_bounds())
    }

    fn row_groups(&self) -> RowGroupIter<Range<usize>, Range<usize>> {
        RowGroupIter::new(self.vertical_bounds(), self.horizontal_bounds())
    }

    fn column_groups(&self) -> ColumnGroupIter<Range<usize>, Range<usize>> {
        ColumnGroupIter::new(self.horizontal_bounds(), self.vertical_bounds())
    }

    fn row_locations(&self, row: usize) -> RowIter<Range<usize>, Range<usize>> {
        RowIter::new(row..(row + 1), self.horizontal_bounds())
    }

    fn column_locations(&self, column: usize) -> RowIter<Range<usize>, Range<usize>> {
        RowIter::new(self.vertical_bounds(), column..(column + 1))
    }

    fn first_row_locations(&self) -> RowIter<Range<usize>, Range<usize>> {
        self.row_locations(self.top())
    }

    fn last_row_locations(&self) -> RowIter<Range<usize>, Range<usize>> {
        self.row_locations(self.bottom())
    }

    fn first_column_locations(&self) -> RowIter<Range<usize>, Range<usize>> {
        self.column_locations(self.left())
    }

    fn last_column_locations(&self) -> RowIter<Range<usize>, Range<usize>> {
        self.column_locations(self.right())
    }

    fn edge_locations(&self, edge: &Direction) -> RowIter<Range<usize>, Range<usize>> {
        use Direction::*;
        match edge {
            North => self.first_row_locations(),
            South => self.last_row_locations(),
            West => self.first_column_locations(),
            East => self.last_column_locations(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::geometry::DIRECTIONS;

    use super::*;

    struct Foo;

    impl AxesBounds for Foo {
        fn vertical_bounds(&self) -> Range<usize> {
            1..4
        }

        fn horizontal_bounds(&self) -> Range<usize> {
            2..4
        }
    }

    #[test]
    fn contains() {
        let foo = Foo;

        for location in foo.locations_by_row() {
            assert!(foo.contains(&location))
        }

        for direction in DIRECTIONS {
            for location in foo.edge_locations(&direction) {
                let outside = location.neighbor(&direction).unwrap();
                assert!(!foo.contains(&outside));
            }
        }
    }

    #[test]
    fn neighbor() {
        let foo = Foo;

        macro_rules! assert_neighbor {
            ($location:expr, $direction:expr, $adjacent:expr) => {
                assert_eq!(
                    foo.neighbor(&$location.into(), &$direction),
                    $adjacent.map(|pair: (usize, usize)| pair.into())
                );
            };
        }

        // Locations just outside of the border should be out of bounds.
        for direction in DIRECTIONS {
            for location in foo.edge_locations(&direction) {
                assert_neighbor!(location, direction, None);
                let outside = location.neighbor(&direction).unwrap();
                assert_neighbor!(outside, direction.inverse(), Some(location.into()));
            }
        }
    }
}
