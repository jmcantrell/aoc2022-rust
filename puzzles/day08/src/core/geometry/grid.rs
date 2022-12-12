use std::convert::TryFrom;
use std::ops::Add;

use anyhow::ensure;

use super::{Direction, Location, Size};

pub type Cell<'a, T> = (Location, &'a T);

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub size: Size,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    fn get_location_index(&self, location: &Location) -> usize {
        location.row * self.size.width + location.column
    }

    fn get_index_location(&self, index: usize) -> Location {
        let row = index / self.size.width;
        let column = index % self.size.width;
        Location { row, column }
    }

    pub fn contains(&self, location: &Location) -> bool {
        location.row < self.size.height && location.column < self.size.width
    }

    pub fn get(&self, location: &Location) -> Option<&T> {
        if self.contains(location) {
            self.cells.get(self.get_location_index(location))
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, location: &Location) -> Option<&mut T> {
        if self.contains(location) {
            let i = self.get_location_index(location);
            self.cells.get_mut(i)
        } else {
            None
        }
    }

    pub fn adjacent_to(&self, location: &Location, direction: &Direction) -> Option<Cell<T>> {
        let adjacent = location.add(*direction)?;
        Some((adjacent, self.get(&adjacent)?))
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.cells.iter()
    }

    pub fn locations(&self) -> impl Iterator<Item = Location> + '_ {
        (0..self.cells.len()).map(|i| self.get_index_location(i))
    }

    pub fn cells(&self) -> impl Iterator<Item = Cell<T>> {
        self.locations().zip(self.values())
    }

    pub fn row(&self, i: usize) -> impl Iterator<Item = Cell<T>> {
        self.cells().skip(self.size.width * i).take(self.size.width)
    }

    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = Cell<T>>> {
        (0..self.size.height).map(|row| self.row(row))
    }

    pub fn column(&self, i: usize) -> impl Iterator<Item = Cell<T>> {
        self.cells().skip(i).step_by(self.size.width)
    }

    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = Cell<T>>> {
        (0..self.size.width).map(|column| self.column(column))
    }

    pub fn vector<'a>(
        &'a self,
        start: &Location,
        direction: &'a Direction,
    ) -> impl Iterator<Item = Cell<T>> + 'a {
        let mut next = self.get(start).map(|value| (*start, value));

        std::iter::from_fn(move || {
            let current = next?;
            next = self.adjacent_to(&current.0, direction);
            Some(current)
        })
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T> {
    type Error = anyhow::Error;

    fn try_from(source: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let height = source.len();

        ensure!(height > 0, "no rows");

        let width = source[0].len();

        for i in 0..height {
            let row_width = source[i].len();
            ensure!(
                row_width == width,
                "row number {} width should be {}, but was {}",
                i + 1,
                width,
                row_width
            );
        }

        let size = Size { width, height };
        let cells = source.into_iter().flatten().collect();

        Ok(Self { size, cells })
    }
}

impl<T: Clone> Grid<T> {
    pub fn filled(size: Size, value: T) -> Self {
        Self {
            size,
            cells: vec![value; size.width * size.height],
        }
    }
}

impl<T: Default + Clone> Grid<T> {
    pub fn sized(size: Size) -> Self {
        Self {
            size,
            cells: vec![T::default(); size.width * size.height],
        }
    }
}
