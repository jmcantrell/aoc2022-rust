use std::convert::TryFrom;
use std::fmt;

use anyhow::{ensure, Context};

use crate::core::{GraphGrid, Location, DIRECTIONS};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Unit {
    Rock,
    Sand,
}

const AIR: &'static str = ".";
const ROCK: &'static str = "#";
const SAND: &'static str = "o";

#[derive(Debug, Clone)]
pub struct CaveMap {
    grid: GraphGrid<Unit>,
    pub floor: Option<isize>,
}

impl CaveMap {
    fn can_drop_to(&self, location: &Location) -> Option<bool> {
        if self.grid.contains(location) {
            return Some(false);
        }

        if let Some(floor) = self.floor {
            if location.row >= floor {
                return Some(false);
            }
        } else if location.row >= self.lowest_rock() {
            return None;
        }

        Some(true)
    }

    pub fn lowest_rock(&self) -> isize {
        self.grid.bottom()
    }

    pub fn drop_sand(&mut self, start: &Location) -> Option<Location> {
        if self.grid.contains(start) {
            return None;
        }

        let mut location = *start;

        'drop: loop {
            for direction in DIRECTIONS.iter() {
                let below = location + *direction;

                match self.can_drop_to(&below) {
                    Some(true) => {
                        location = below;
                        continue 'drop;
                    }
                    Some(false) => {
                        continue;
                    }
                    None => {
                        return None;
                    }
                }
            }

            self.grid.insert(location, Unit::Sand);
            return Some(location);
        }
    }

    pub fn fill_with_sand<'a>(
        &'a mut self,
        start: &'a Location,
    ) -> impl Iterator<Item = Location> + '_ {
        std::iter::from_fn(|| self.drop_sand(start))
    }
}

impl fmt::Display for CaveMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (top_left, mut bottom_right) = self.grid.extents();

        if let Some(floor) = self.floor {
            bottom_right.row = floor;
        }

        for row in top_left.row..=bottom_right.row {
            for column in top_left.column..=bottom_right.column {
                let mut s = AIR;

                if let Some(unit) = self.grid.get(&Location { row, column }) {
                    s = match unit {
                        Unit::Rock => ROCK,
                        Unit::Sand => SAND,
                    };
                } else if let Some(floor) = self.floor {
                    if row == floor {
                        s = ROCK;
                    }
                };

                write!(f, "{}", s)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl TryFrom<&str> for CaveMap {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        fn parse_location(s: &str) -> anyhow::Result<Location> {
            s.try_into()
                .with_context(|| format!("invalid location: {:?}", s))
        }

        fn parse_path(s: &str) -> anyhow::Result<Vec<Location>> {
            s.split("->")
                .enumerate()
                .map(|(i, s)| {
                    parse_location(s.trim()).with_context(|| format!("location number {}", i + 1))
                })
                .collect::<Result<Vec<_>, _>>()
        }

        fn stroke_path(s: &str) -> anyhow::Result<Vec<Location>> {
            let mut path = parse_path(s)?;

            ensure!(path.len() > 0, "empty path");

            let mut prev = path.pop().unwrap();

            let mut stroke = Vec::new();

            while !path.is_empty() {
                let next = path.pop().unwrap();
                let unit = (next - prev).unit();
                let mut current = prev;
                while current != next {
                    stroke.push(current);
                    current = current + unit;
                }
                prev = next;
            }

            stroke.push(prev);

            Ok(stroke)
        }

        let mut grid = GraphGrid::new();

        for (i, line) in s.lines().enumerate() {
            let path = stroke_path(line).with_context(|| format!("path number {}", i + 1))?;
            for location in path.into_iter() {
                grid.insert(location, Unit::Rock);
            }
        }

        Ok(Self { grid, floor: None })
    }
}
