use std::fmt;
use std::ops::Range;

use anyhow::Context;

use crate::geometry::{AxesBounds, Direction, Grid, Location};

use super::{Tile, Walker};

#[derive(Debug, Clone)]
pub struct Map {
    pub grid: Grid<Option<Tile>>,
}

impl Map {
    pub fn origin(&self) -> Location {
        self.grid
            .row_groups()
            .next()
            .expect("no rows")
            .find(|location| self.grid[&location].is_some())
            .expect("no non-empty tiles in the first row")
    }

    pub fn walker(&self) -> Walker {
        Walker::new(self.clone(), self.origin(), Direction::East)
    }
}

impl AxesBounds for Map {
    fn vertical_bounds(&self) -> Range<usize> {
        self.grid.vertical_bounds()
    }

    fn horizontal_bounds(&self) -> Range<usize> {
        self.grid.horizontal_bounds()
    }
}

impl From<Grid<Option<Tile>>> for Map {
    fn from(grid: Grid<Option<Tile>>) -> Self {
        Self { grid }
    }
}

impl From<Map> for Grid<Option<Tile>> {
    fn from(map: Map) -> Grid<Option<Tile>> {
        map.grid
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.grid.row_groups() {
            for location in row {
                match self.grid.get(&location).unwrap().as_ref() {
                    Some(tile) => {
                        write!(f, "{tile}")?;
                    }
                    None => {
                        write!(f, " ")?;
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl TryFrom<Vec<&str>> for Map {
    type Error = anyhow::Error;

    fn try_from(lines: Vec<&str>) -> Result<Self, Self::Error> {
        fn parse_row(s: &str) -> anyhow::Result<Vec<Option<Tile>>> {
            s.chars()
                .enumerate()
                .map(|(i, c)| {
                    if c == ' ' {
                        Ok(None)
                    } else {
                        Ok(Some(
                            c.try_into()
                                .with_context(|| format!("column number {}", i + 1))?,
                        ))
                    }
                })
                .collect::<Result<Vec<_>, _>>()
        }

        let mut grid = lines
            .into_iter()
            .enumerate()
            .map(|(i, s)| parse_row(s).with_context(|| format!("line number {}", i + 1)))
            .collect::<Result<Vec<Vec<_>>, _>>()?;

        let width = grid.iter().map(|row| row.len()).max().unwrap_or_default();

        for row in grid.iter_mut() {
            while row.len() < width {
                row.push(None);
            }
        }

        let grid: Grid<Option<Tile>> = grid.try_into()?;

        Ok(grid.into())
    }
}
