use crate::core::geometry::{Direction, Location};

use super::{Map, Movement, Tile};

#[derive(Debug, Clone)]
pub struct Walker {
    pub map: Map,
    pub location: Location,
    pub direction: Direction,
}

impl Walker {
    pub fn new(map: Map, location: Location, direction: Direction) -> Self {
        Self {
            map,
            location,
            direction,
        }
    }

    pub fn record(&mut self) {
        *self.map.grid.get_some_mut(&self.location).unwrap() = Tile::Trail(self.direction);
    }

    pub fn password(&self) -> usize {
        1000 * (self.location.row + 1)
            + 4 * (self.location.column + 1)
            + match self.direction {
                Direction::East => 0,
                Direction::South => 1,
                Direction::West => 2,
                Direction::North => 3,
            }
    }
}

pub trait Walk<'a> {
    fn walker(&self) -> Walker;

    fn neighbor(&self, location: Location, direction: Direction) -> (Location, Direction, Tile);

    fn walk(&self, path: &[Movement]) -> Walker {
        let mut walker = self.walker();
        walker.record();

        for movement in path {
            match movement {
                Movement::Rotate(rotation) => {
                    walker.direction += *rotation;
                    walker.record();
                }
                Movement::Forward(distance) => {
                    for _ in 0..*distance {
                        let (adjacent, direction, tile) =
                            self.neighbor(walker.location, walker.direction);

                        if tile != Tile::Wall {
                            walker.location = adjacent;
                            walker.direction = direction;
                            walker.record();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        walker
    }
}