use std::array::IntoIter;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::iter::Cycle;

use anyhow::{ensure, Context};

use super::Location;

pub type Choices = [Location; 3];

const NORTH: Location = [-1, 0];
const SOUTH: Location = [1, 0];
const WEST: Location = [0, -1];
const EAST: Location = [0, 1];

const fn add_locations(a: Location, b: Location) -> Location {
    [a[0] + b[0], a[1] + b[1]]
}

const NORTH_WEST: Location = add_locations(NORTH, WEST);
const NORTH_EAST: Location = add_locations(NORTH, EAST);
const SOUTH_WEST: Location = add_locations(SOUTH, WEST);
const SOUTH_EAST: Location = add_locations(SOUTH, EAST);

const DIRECTIONS: [Choices; 4] = [
    [NORTH, NORTH_EAST, NORTH_WEST],
    [SOUTH, SOUTH_EAST, SOUTH_WEST],
    [WEST, NORTH_WEST, SOUTH_WEST],
    [EAST, NORTH_EAST, SOUTH_EAST],
];

#[derive(Debug, Clone)]
pub struct Grove {
    elves: HashSet<Location>,
    directions: Cycle<IntoIter<Choices, 4>>,
}

impl Grove {
    pub fn new(elves: HashSet<Location>) -> Self {
        if elves.is_empty() {
            panic!("no elves");
        }

        Self {
            elves,
            directions: DIRECTIONS.into_iter().cycle(),
        }
    }

    fn extents(&self) -> (Location, Location) {
        let mut elves = self.elves.clone().into_iter();

        let mut top_left = elves.next().unwrap();
        let mut bottom_right = top_left;

        for elf in elves {
            if elf[0] < top_left[0] {
                top_left[0] = elf[0];
            }
            if elf[0] > bottom_right[0] {
                bottom_right[0] = elf[0];
            }
            if elf[1] < top_left[1] {
                top_left[1] = elf[1];
            }
            if elf[1] > bottom_right[1] {
                bottom_right[1] = elf[1];
            }
        }

        (top_left, bottom_right)
    }

    pub fn count_empty_tiles(&self) -> usize {
        let (top_left, bottom_right) = self.extents();

        let height = bottom_right[0].abs_diff(top_left[0]) + 1;
        let width = bottom_right[1].abs_diff(top_left[1]) + 1;

        height * width - self.elves.len()
    }

    pub fn iterate(&mut self) -> bool {
        let mut proposals: HashMap<Location, Vec<Location>> = HashMap::new();

        for &elf in self.elves.iter() {
            let mut possibilities = Vec::with_capacity(DIRECTIONS.len());

            for choices in self.directions.clone().take(DIRECTIONS.len()) {
                if choices
                    .into_iter()
                    .all(|choice| !self.elves.contains(&add_locations(elf, choice)))
                {
                    possibilities.push(add_locations(elf, choices[0]));
                }
            }

            if !possibilities.is_empty() && possibilities.len() != DIRECTIONS.len() {
                proposals.entry(possibilities[0]).or_default().push(elf);
            }
        }

        let mut any_moved = false;

        for (dest, elves) in proposals.into_iter() {
            if elves.len() == 1 {
                any_moved = true;
                self.elves.remove(&elves[0]);
                self.elves.insert(dest);
            }
        }

        self.directions.next().unwrap();

        any_moved
    }
}

impl fmt::Display for Grove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ([top, left], [bottom, right]) = self.extents();

        for row in top..=bottom {
            for column in left..=right {
                write!(
                    f,
                    "{}",
                    if self.elves.contains(&[row, column]) {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl From<HashSet<Location>> for Grove {
    fn from(elves: HashSet<Location>) -> Self {
        Self::new(elves)
    }
}

impl TryFrom<&str> for Grove {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut elves = HashSet::new();

        for (i, line) in s.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }

                ensure!(
                    c == '#',
                    "unrecognized character at line {} and column {}: {:?}",
                    i + 1,
                    j + 1,
                    c
                );

                let row = isize::try_from(i).context("too many rows")?;
                let column = isize::try_from(j).context("too many columns")?;

                elves.insert([row, column]);
            }
        }

        Ok(elves.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = "##\n#.\n..\n##\n";

    #[test]
    fn test_identity() {
        assert_eq!(
            Grove::try_from(SMALL_EXAMPLE).unwrap().to_string(),
            SMALL_EXAMPLE
        );
    }
}
