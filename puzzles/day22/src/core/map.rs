use std::fmt;

use anyhow::Context;

use super::{CardinalDirection, Grid, Location, Tile, Walker};

use CardinalDirection::*;

#[derive(Debug, Clone)]
pub struct Map {
    pub grid: Grid<Option<Tile>>,
}

impl Map {
    pub fn origin(&self) -> Location {
        self.grid
            .row_iter()
            .enumerate()
            .find_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(j, value)| value.is_some().then_some((i, j)))
            })
            .expect("grid is empty")
    }

    pub fn walker(&self) -> Walker {
        Walker::new(self.clone(), self.origin(), East)
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.grid.row_iter() {
            for maybe_tile in row.iter() {
                match maybe_tile {
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

        let mut rows = lines
            .into_iter()
            .enumerate()
            .map(|(i, s)| parse_row(s).with_context(|| format!("line number {}", i + 1)))
            .collect::<Result<Vec<Vec<_>>, _>>()?;

        let height = rows.len();
        let width = rows.iter().map(|row| row.len()).max().unwrap_or_default();

        for row in rows.iter_mut() {
            while row.len() < width {
                row.push(None);
            }
        }

        let values = rows.into_iter().flat_map(|row| row.into_iter());

        let grid: Grid<Option<Tile>> = Grid::from_row_iterator(height, width, values);

        Ok(Self { grid })
    }
}
