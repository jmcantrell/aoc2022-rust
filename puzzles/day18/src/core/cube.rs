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
        (self.min.x..=self.max.x).flat_map(move |x| {
            (self.min.y..=self.max.y)
                .flat_map(move |y| (self.min.z..=self.max.z).map(move |z| Point::new(x, y, z)))
        })
    }

    pub fn to_grid(&self) -> CubeGrid {
        CubeGrid::from(self.points().collect::<HashSet<_>>())
    }
}

impl Add<isize> for Cube {
    type Output = Self;

    fn add(self, n: isize) -> Self {
        Self {
            min: self.min.map(|c| c - n),
            max: self.max.map(|c| c + n),
        }
    }
}
