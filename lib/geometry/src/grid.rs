use std::convert::TryFrom;
use std::fmt;
use std::ops::{Index, IndexMut, Range};

use anyhow::ensure;

use super::{AxesBounds, CardinalDirection, Location, Rectangle, Size, VectorIterator};

pub type GridLocation = Location<usize>;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    values: Vec<T>,
    size: Size<usize>,
}

pub fn index_for_location(location: &GridLocation, width: usize) -> usize {
    location.y * width + location.x
}

impl<T> Grid<T> {
    fn index_for_location(&self, location: &GridLocation) -> usize {
        index_for_location(location, self.width())
    }

    pub fn contains(&self, location: &GridLocation) -> bool {
        location.y < self.height() && location.x < self.width()
    }

    pub fn get(&self, location: &GridLocation) -> Option<&T> {
        if self.contains(location) {
            let i = self.index_for_location(location);
            self.values.get(i)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, location: &GridLocation) -> Option<&mut T> {
        if self.contains(location) {
            let i = self.index_for_location(location);
            self.values.get_mut(i)
        } else {
            None
        }
    }

    pub fn values(&self) -> impl ExactSizeIterator + DoubleEndedIterator<Item = &T> {
        self.values.iter()
    }

    pub fn swap(&mut self, a: &GridLocation, b: &GridLocation) {
        let i = self.index_for_location(a);
        let j = self.index_for_location(b);
        self.values.swap(i, j);
    }
}

impl<T> Grid<Option<T>> {
    pub fn get_some(&self, location: &GridLocation) -> Option<&T> {
        self.get(location).and_then(|maybe| maybe.as_ref())
    }

    pub fn get_some_mut(&mut self, location: &GridLocation) -> Option<&mut T> {
        self.get_mut(location).and_then(|maybe| maybe.as_mut())
    }

    pub fn take_some(&mut self, location: &GridLocation) -> Option<T> {
        self.get_mut(location).and_then(|maybe| maybe.take())
    }

    pub fn contains_some(&self, location: &GridLocation) -> bool {
        self.get_some(location).is_some()
    }

    pub fn has_some_neighbor(
        &self,
        location: &GridLocation,
        direction: &CardinalDirection,
    ) -> bool {
        self.neighbor_some(location, direction).is_some()
    }

    pub fn neighbor_some(
        &self,
        location: &GridLocation,
        direction: &CardinalDirection,
    ) -> Option<GridLocation> {
        self.neighbor(location, direction)
            .and_then(|adjacent| self.contains_some(&adjacent).then_some(adjacent))
    }

    fn next_non_empty_location(
        &self,
        locations: &mut impl VectorIterator<usize>,
    ) -> Option<GridLocation> {
        locations.find_map(|location| self.contains_some(&location).then_some(location))
    }

    pub fn first_row_some(&self) -> Option<usize> {
        self.row_groups()
            .find_map(|mut row| self.next_non_empty_location(&mut row))
            .map(|location| location.y)
    }

    pub fn last_row_some(&self) -> Option<usize> {
        self.row_groups()
            .rev()
            .find_map(|mut row| self.next_non_empty_location(&mut row))
            .map(|location| location.y)
    }

    pub fn first_column_some(&self) -> Option<usize> {
        self.column_groups()
            .find_map(|mut column| self.next_non_empty_location(&mut column))
            .map(|location| location.x)
    }

    pub fn last_column_some(&self) -> Option<usize> {
        self.column_groups()
            .rev()
            .find_map(|mut column| self.next_non_empty_location(&mut column))
            .map(|location| location.x)
    }

    pub fn bounds_some(&self) -> Option<Rectangle<usize>> {
        let top = self.first_row_some()?;
        let bottom = self.last_row_some()?;
        let left = self.first_column_some()?;
        let right = self.last_column_some()?;

        let top_left = Location::new(left, top);
        let bottom_right = Location::new(right, bottom);

        Some(Rectangle::new(top_left, bottom_right))
    }

    pub fn vertical_bounds_by_column(
        &self,
    ) -> impl Iterator<Item = (GridLocation, GridLocation)> + '_ {
        self.column_groups().filter_map(|mut column| {
            let first = self.next_non_empty_location(&mut column)?;
            let last = self
                .next_non_empty_location(&mut column.rev())
                .unwrap_or(first);
            Some((first, last))
        })
    }

    pub fn horizontal_bounds_by_row(
        &self,
    ) -> impl Iterator<Item = (GridLocation, GridLocation)> + '_ {
        self.row_groups().filter_map(|mut row| {
            let first = self.next_non_empty_location(&mut row)?;
            let last = self
                .next_non_empty_location(&mut row.rev())
                .unwrap_or(first);
            Some((first, last))
        })
    }
}

impl<T> AxesBounds<usize> for Grid<T> {
    fn vertical_bounds(&self) -> Range<usize> {
        0..self.size.y
    }

    fn horizontal_bounds(&self) -> Range<usize> {
        0..self.size.x
    }
}

impl<T> Index<&GridLocation> for Grid<T> {
    type Output = T;

    fn index(&self, location: &GridLocation) -> &Self::Output {
        let i = self.index_for_location(location);
        &self.values[i]
    }
}

impl<T> IndexMut<&GridLocation> for Grid<T> {
    fn index_mut(&mut self, location: &GridLocation) -> &mut Self::Output {
        let i = self.index_for_location(location);
        &mut self.values[i]
    }
}

pub struct GridIntoIter<T> {
    values: std::vec::IntoIter<T>,
    locations: std::vec::IntoIter<GridLocation>,
}

impl<T> GridIntoIter<T> {
    pub fn new(grid: Grid<T>) -> Self {
        let locations = grid.row_major_locations().collect::<Vec<_>>().into_iter();
        let values = grid.values.into_iter();
        Self { values, locations }
    }
}

impl<T> From<Grid<T>> for GridIntoIter<T> {
    fn from(grid: Grid<T>) -> Self {
        Self::new(grid)
    }
}

impl<T> Iterator for GridIntoIter<T> {
    type Item = (GridLocation, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.locations
            .next()
            .map(|location| (location, self.values.next().unwrap()))
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = (GridLocation, T);
    type IntoIter = GridIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<T> TryFrom<(Size<usize>, Vec<T>)> for Grid<T> {
    type Error = anyhow::Error;

    fn try_from((size, values): (Size<usize>, Vec<T>)) -> Result<Self, Self::Error> {
        let expected_len = size.area();

        ensure!(
            expected_len == values.len(),
            "expected {} values, but got {}",
            expected_len,
            values.len()
        );

        Ok(Self { size, values })
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T> {
    type Error = anyhow::Error;

    fn try_from(source: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let height = source.len();
        let width = source.iter().map(|row| row.len()).max().unwrap_or_default();

        for (i, row) in source.iter().enumerate() {
            ensure!(
                row.len() == width,
                "row number {} width should be {}, but was {}",
                i + 1,
                width,
                row.len()
            );
        }

        let size = Size::new(width, height);
        let values = source.into_iter().flatten().collect();

        Ok(Self { size, values })
    }
}

impl<T: Clone> Grid<T> {
    pub fn filled(size: Size<usize>, value: T) -> Self {
        Self {
            size,
            values: vec![value; size.area()],
        }
    }
}

impl<T: Default + Clone> Grid<T> {
    pub fn sized(size: Size<usize>) -> Self {
        Self {
            size,
            values: vec![T::default(); size.area()],
        }
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.row_groups() {
            for location in row {
                write!(f, "{}", self[&location])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
