use std::collections::HashSet;
use std::ops::Add;

use super::{CubeGrid, Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cube {
    pub min: Point,
    pub max: Point,
}

impl Cube {
    pub fn points(&self) -> impl Iterator<Item = Point> + '_ {
        (self.min.x..=self.max.x)
            .map(move |x| {
                (self.min.y..=self.max.y)
                    .map(move |y| (self.min.z..=self.max.z).map(move |z| Point { x, y, z }))
                    .flatten()
            })
            .flatten()
    }

    pub fn to_grid(&self) -> CubeGrid {
        CubeGrid::from(self.points().collect::<HashSet<_>>())
    }
}

impl Add<isize> for Cube {
    type Output = Self;

    fn add(self, n: isize) -> Self {
        let increment = Point { x: n, y: n, z: n };

        Self {
            min: self.min - increment,
            max: self.max + increment,
        }
    }
}
