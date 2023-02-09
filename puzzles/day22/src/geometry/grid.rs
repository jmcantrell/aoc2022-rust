use std::convert::TryFrom;
use std::fmt;
use std::ops::{Index, IndexMut, Range};

use anyhow::ensure;

use super::{AxesBounds, Direction, Location, LocationIterator, Rectangle, Size};

pub type Cell<'a, T> = (Location, &'a T);

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub size: Size,
    values: Vec<T>,
}

pub fn index_for_location(location: &Location, width: usize) -> usize {
    location.row * width + location.column
}

impl<T> Grid<T> {
    fn index_for_location(&self, location: &Location) -> usize {
        index_for_location(location, self.width())
    }

    pub fn get(&self, location: &Location) -> Option<&T> {
        if self.contains(location) {
            let i = self.index_for_location(location);
            self.values.get(i)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, location: &Location) -> Option<&mut T> {
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

    pub fn cells(&self) -> impl ExactSizeIterator + DoubleEndedIterator<Item = Cell<T>> {
        self.locations_by_row().zip(self.values())
    }

    pub fn neighbor_cell(&self, location: &Location, direction: &Direction) -> Option<Cell<T>> {
        self.neighbor(location, direction)
            .and_then(|adjacent| self.get(&adjacent).map(|value| (adjacent, value)))
    }

    pub fn swap(&mut self, a: &Location, b: &Location) {
        let i = self.index_for_location(a);
        let j = self.index_for_location(b);
        self.values.swap(i, j);
    }
}

impl<T> Grid<Option<T>> {
    pub fn get_some(&self, location: &Location) -> Option<&T> {
        self.get(location).and_then(|maybe| maybe.as_ref())
    }

    pub fn get_some_mut(&mut self, location: &Location) -> Option<&mut T> {
        self.get_mut(location).and_then(|maybe| maybe.as_mut())
    }

    pub fn take_some(&mut self, location: &Location) -> Option<T> {
        self.get_mut(location).and_then(|maybe| maybe.take())
    }

    pub fn contains_some(&self, location: &Location) -> bool {
        self.get_some(location).is_some()
    }

    pub fn has_some_neighbor(&self, location: &Location, direction: &Direction) -> bool {
        self.neighbor_some(location, direction).is_some()
    }

    pub fn neighbor_some(&self, location: &Location, direction: &Direction) -> Option<Location> {
        self.neighbor(location, direction)
            .and_then(|adjacent| self.contains_some(&adjacent).then_some(adjacent))
    }

    pub fn neighbor_cell_some(
        &self,
        location: &Location,
        direction: &Direction,
    ) -> Option<(Location, &T)> {
        self.neighbor_cell(location, direction)
            .and_then(|(adjacent, maybe)| maybe.as_ref().map(|value| (adjacent, value)))
    }

    fn next_non_empty_location(&self, locations: &mut impl LocationIterator) -> Option<Location> {
        locations.find_map(|location| self.contains_some(&location).then_some(location))
    }

    pub fn first_row_some(&self) -> Option<usize> {
        self.row_groups()
            .find_map(|mut row| self.next_non_empty_location(&mut row))
            .map(|location| location.row)
    }

    pub fn last_row_some(&self) -> Option<usize> {
        self.row_groups()
            .rev()
            .find_map(|mut row| self.next_non_empty_location(&mut row))
            .map(|location| location.row)
    }

    pub fn first_column_some(&self) -> Option<usize> {
        self.column_groups()
            .find_map(|mut column| self.next_non_empty_location(&mut column))
            .map(|location| location.column)
    }

    pub fn last_column_some(&self) -> Option<usize> {
        self.column_groups()
            .rev()
            .find_map(|mut column| self.next_non_empty_location(&mut column))
            .map(|location| location.column)
    }

    pub fn bounds_some(&self) -> Option<Rectangle> {
        let top = self.first_row_some()?;
        let bottom = self.last_row_some()?;
        let left = self.first_column_some()?;
        let right = self.last_column_some()?;

        let top_left = Location::new(top, left);
        let bottom_right = Location::new(bottom, right);

        Some(Rectangle::new(top_left, bottom_right))
    }

    pub fn vertical_bounds_by_column(&self) -> impl Iterator<Item = (Location, Location)> + '_ {
        self.column_groups().filter_map(|mut column| {
            let first = self.next_non_empty_location(&mut column)?;
            let last = self
                .next_non_empty_location(&mut column.rev())
                .unwrap_or(first);
            Some((first, last))
        })
    }

    pub fn horizontal_bounds_by_row(&self) -> impl Iterator<Item = (Location, Location)> + '_ {
        self.row_groups().filter_map(|mut row| {
            let first = self.next_non_empty_location(&mut row)?;
            let last = self
                .next_non_empty_location(&mut row.rev())
                .unwrap_or(first);
            Some((first, last))
        })
    }
}

impl<T> AxesBounds for Grid<T> {
    fn vertical_bounds(&self) -> Range<usize> {
        self.size.vertical_bounds()
    }

    fn horizontal_bounds(&self) -> Range<usize> {
        self.size.horizontal_bounds()
    }
}

impl<T> Index<&Location> for Grid<T> {
    type Output = T;

    fn index(&self, location: &Location) -> &Self::Output {
        let i = self.index_for_location(location);
        &self.values[i]
    }
}

impl<T> IndexMut<&Location> for Grid<T> {
    fn index_mut(&mut self, location: &Location) -> &mut Self::Output {
        let i = self.index_for_location(location);
        &mut self.values[i]
    }
}

pub struct GridIntoIter<T> {
    values: std::vec::IntoIter<T>,
    locations: std::vec::IntoIter<Location>,
}

impl<T> GridIntoIter<T> {
    pub fn new(grid: Grid<T>) -> Self {
        let locations = grid.locations_by_row().collect::<Vec<_>>().into_iter();
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
    type Item = (Location, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.locations
            .next()
            .map(|location| (location, self.values.next().unwrap()))
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = (Location, T);
    type IntoIter = GridIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<T> TryFrom<(Size, Vec<T>)> for Grid<T> {
    type Error = anyhow::Error;

    fn try_from((size, values): (Size, Vec<T>)) -> Result<Self, Self::Error> {
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

        let size = Size { width, height };
        let values = source.into_iter().flatten().collect();

        Ok(Self { size, values })
    }
}

impl<T: Clone> Grid<T> {
    pub fn filled(size: Size, value: T) -> Self {
        Self {
            size,
            values: vec![value; size.area()],
        }
    }
}

impl<T: Default + Clone> Grid<T> {
    pub fn sized(size: Size) -> Self {
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
