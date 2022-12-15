use std::ops::Add;

use anyhow::ensure;

use super::{Direction, Location, Size, DIRECTIONS};

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

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.cells.iter()
    }

    pub fn locations(&self) -> impl Iterator<Item = Location> + '_ {
        (0..self.cells.len()).map(|i| self.get_index_location(i))
    }

    pub fn cells(&self) -> impl Iterator<Item = Cell<T>> {
        self.locations().zip(self.values())
    }

    pub fn adjacent_to(&self, location: &Location, direction: &Direction) -> Option<Cell<T>> {
        let adjacent = location.add(*direction)?;
        Some((adjacent, self.get(&adjacent)?))
    }

    pub fn neighbors<'a>(&'a self, location: &'a Location) -> impl Iterator<Item = Cell<T>> + 'a {
        DIRECTIONS
            .iter()
            .filter_map(|direction| self.adjacent_to(location, direction))
    }
}

impl<T> Grid<T> {
    pub fn try_from_cells(cells: Vec<T>, width: usize) -> anyhow::Result<Self> {
        ensure!(width > 0, "width must be a positive integer");

        let n = cells.len();

        ensure!(n > 0, "no cells");

        ensure!(n % width == 0, "uneven number of cells");

        let size = Size {
            width,
            height: n / width,
        };

        Ok(Self { size, cells })
    }
}
