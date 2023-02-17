use std::collections::HashMap;
use std::fmt;

use anyhow::{bail, ensure, Context};

use super::{Direction, Grid, Location, MapIter, Tile};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    pub grid: Grid<Tile>,
    pub blizzards: HashMap<Location, Direction>,
    pub start: Location,
    pub end: Location,
}

impl Map {
    pub fn iter(&self) -> MapIter {
        MapIter::new(self)
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.grid.row_iter() {
            for tile in row.iter() {
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl TryFrom<&str> for Map {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut values = Vec::new();
        let mut lines = s.lines().peekable();

        let width = lines.peek().context("map has no rows")?.len();

        ensure!(width > 2, "map is too narrow");

        let mut blizzards = HashMap::new();

        for (i, line) in lines.enumerate() {
            ensure!(
                line.len() == width,
                "expected length of line number {} to be {}, but it was {} instead",
                i + 1,
                width,
                line.len()
            );

            for (j, c) in line.chars().enumerate() {
                values.push(if let Ok(direction) = Direction::try_from(c) {
                    blizzards.insert(Location::new(i, j), direction);
                    Tile::Floor
                } else {
                    Tile::try_from(c).with_context(|| {
                        format!("line number {} and column number {}", i + 1, j + 1)
                    })?
                });
            }
        }

        let height = values.len() / width;

        let grid = Grid::from_row_iterator(height, width, values.into_iter());

        fn ensure_one_floor_tile(grid: &Grid<Tile>, row: usize) -> anyhow::Result<Location> {
            let view = grid.row(row);
            let mut iter = view.iter().enumerate();

            let column = iter
                .find_map(|(column, tile)| (tile == &Tile::Floor).then_some(column))
                .with_context(|| format!("no floor location on row number {}", row + 1))?;

            let location = Location::new(row, column);

            if let Some((column, tile)) = iter.find(|(_, tile)| tile != &&Tile::Wall) {
                bail!(
                    "expected non-wall tile on row number {} and column number {}, but got: {}",
                    row + 1,
                    column + 1,
                    tile
                );
            }

            Ok(location)
        }

        let start = ensure_one_floor_tile(&grid, 0)?;
        let end = ensure_one_floor_tile(&grid, grid.nrows() - 1)?;

        for row in 1..(grid.nrows() - 1) {
            for column in [0, grid.ncols() - 1] {
                let tile = grid[(row, column)];
                ensure!(
                    tile == Tile::Wall,
                    "expected a wall at row number {} and column number {}, but it was: {}",
                    row + 1,
                    column + 1,
                    tile
                );
            }

            for column in 1..(grid.ncols() - 1) {
                let tile = grid[(row, column)];
                ensure!(
                    tile != Tile::Wall,
                    "expected something other than a wall at row number {} and column number {}, but it was: {}",
                    row + 1, column + 1, tile
                );
            }
        }

        Ok(Self {
            grid,
            blizzards,
            start,
            end,
        })
    }
}
