use super::{CardinalDirection, Location, Map, Movement, Tile};

use Tile::*;

#[derive(Debug, Clone)]
pub struct Walker {
    pub map: Map,
    pub location: Location,
    pub direction: CardinalDirection,
}

impl Walker {
    pub fn new(map: Map, location: Location, direction: CardinalDirection) -> Self {
        Self {
            map,
            location,
            direction,
        }
    }

    pub fn neighbor(&self) -> Option<(Location, Tile)> {
        self.direction.neighbor(self.location).and_then(|loc| {
            self.map
                .grid
                .get(loc)
                .and_then(|val| val.map(|val| (loc, val)))
        })
    }

    pub fn record(&mut self) {
        self.map.grid[self.location] = Some(Tile::Trail(self.direction));
    }

    pub fn password(&self) -> usize {
        1000 * (self.location.0 + 1) + 4 * (self.location.1 + 1) + self.direction.value()
    }
}

pub trait Walk<'a> {
    fn walker(&self) -> Walker;

    fn portal(&self, loc: Location, dir: CardinalDirection) -> (Location, CardinalDirection, Tile);

    fn step(&self, walker: &mut Walker) -> bool {
        let (loc, dir, tile) = match walker.neighbor() {
            Some((loc, tile)) => (loc, walker.direction, tile),
            None => self.portal(walker.location, walker.direction),
        };

        if tile == Wall {
            return false;
        }

        walker.location = loc;
        walker.direction = dir;
        walker.record();

        true
    }

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
                        if !self.step(&mut walker) {
                            break;
                        }
                    }
                }
            }
        }

        walker
    }
}
