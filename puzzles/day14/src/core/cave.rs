use std::collections::HashMap;
use std::fmt;

use anyhow::{ensure, Context};

use super::{Location, FALLS};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Unit {
    Rock,
    Sand,
}

const AIR: &str = ".";
const ROCK: &str = "#";
const SAND: &str = "o";

#[derive(Debug, Clone)]
pub struct CaveMap {
    grid: HashMap<Location, Unit>,
    pub lowest_rock: isize,
    pub floor: Option<isize>,
}

impl CaveMap {
    fn extents(&self) -> (Location, Location) {
        if self.grid.is_empty() {
            return (Location::default(), Location::default());
        }

        let mut points = self.grid.keys();

        let first = points.next().unwrap();

        let mut top_left = *first;
        let mut bottom_right = *first;

        for &point in points {
            if point.y < top_left.y {
                top_left.y = point.y;
            }
            if point.x < top_left.x {
                top_left.x = point.x;
            }
            if point.y > bottom_right.y {
                bottom_right.y = point.y;
            }
            if point.x > bottom_right.x {
                bottom_right.x = point.x;
            }
        }

        (top_left, bottom_right)
    }

    pub fn bottom(&self) -> isize {
        self.extents().1.y
    }

    fn can_drop_to(&self, point: &Location) -> Option<bool> {
        if self.grid.contains_key(point) {
            return Some(false);
        }

        if let Some(floor) = self.floor {
            if point.y >= floor {
                return Some(false);
            }
        } else if point.y >= self.lowest_rock {
            return None;
        }

        Some(true)
    }

    pub fn drop_sand(&mut self, start: &Location) -> Option<Location> {
        if self.grid.contains_key(start) {
            return None;
        }

        let mut point = *start;

        'fall: loop {
            for fall in FALLS.iter() {
                let below = point + *fall;

                match self.can_drop_to(&below) {
                    Some(true) => {
                        point = below;
                        continue 'fall;
                    }
                    Some(false) => {
                        continue;
                    }
                    None => {
                        return None;
                    }
                }
            }

            self.grid.insert(point, Unit::Sand);
            return Some(point);
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
        let (top_left, mut bottom_right) = self.extents();

        if let Some(floor) = self.floor {
            bottom_right.y = floor;
        }

        for y in top_left.y..=bottom_right.y {
            for x in top_left.x..=bottom_right.x {
                let mut s = AIR;

                if let Some(unit) = self.grid.get(&Location::new(x, y)) {
                    s = match unit {
                        Unit::Rock => ROCK,
                        Unit::Sand => SAND,
                    };
                } else if let Some(floor) = self.floor {
                    if y == floor {
                        s = ROCK;
                    }
                };

                write!(f, "{s}")?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl TryFrom<&str> for CaveMap {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        fn parse_int(s: &str) -> anyhow::Result<isize> {
            s.parse().with_context(|| format!("invalid integer: {s:?}"))
        }

        fn parse_point(s: &str) -> anyhow::Result<Location> {
            let mut tokens = s.trim().splitn(2, ',');

            let x = parse_int(tokens.next().context("missing column")?)?;
            let y = parse_int(tokens.next().context("missing row")?)?;

            Ok(Location::new(x, y))
        }

        fn parse_path(s: &str) -> anyhow::Result<Vec<Location>> {
            s.split("->")
                .enumerate()
                .map(|(i, s)| {
                    parse_point(s.trim()).with_context(|| format!("point number {}", i + 1))
                })
                .collect::<Result<Vec<_>, _>>()
        }

        fn stroke_path(s: &str) -> anyhow::Result<Vec<Location>> {
            let mut path = parse_path(s)?;

            ensure!(!path.is_empty(), "empty path");

            let mut prev = path.pop().unwrap();

            let mut stroke = Vec::new();

            while !path.is_empty() {
                let next = path.pop().unwrap();
                let unit = (next - prev).map(|c| c.signum());
                let mut current = prev;
                while current != next {
                    stroke.push(current);
                    current += unit;
                }
                prev = next;
            }

            stroke.push(prev);

            Ok(stroke)
        }

        let mut grid = HashMap::new();
        let mut lowest_rock = 0;

        for (i, line) in s.lines().enumerate() {
            let path = stroke_path(line).with_context(|| format!("path number {}", i + 1))?;
            for point in path.into_iter() {
                if point.y > lowest_rock {
                    lowest_rock = point.y;
                }
                grid.insert(point, Unit::Rock);
            }
        }

        Ok(Self {
            grid,
            lowest_rock,
            floor: None,
        })
    }
}
