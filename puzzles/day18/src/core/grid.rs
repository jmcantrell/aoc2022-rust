use std::collections::{HashSet, VecDeque};

use super::{Cube, Point, DIRECTIONS};

#[derive(Debug, Clone, Default)]
pub struct CubeGrid {
    pub points: HashSet<Point>,
}

impl CubeGrid {
    pub fn neighbors<'a>(&'a self, point: &'a Point) -> impl Iterator<Item = Point> + '_ {
        DIRECTIONS.into_iter().filter_map(|direction| {
            let neighbor = *point + direction;
            if self.points.contains(&neighbor) {
                Some(neighbor)
            } else {
                None
            }
        })
    }

    pub fn surface_area(&self) -> usize {
        self.points
            .iter()
            .map(|point| 6 - self.neighbors(point).count())
            .sum()
    }

    pub fn bounding_box(&self) -> Option<Cube> {
        let mut points = self.points.iter();

        let first = points.next()?;

        let mut min = *first;
        let mut max = *first;

        for point in points {
            min.x = min.x.min(point.x);
            min.y = min.y.min(point.y);
            min.z = min.z.min(point.z);

            max.x = max.x.max(point.x);
            max.y = max.y.max(point.y);
            max.z = max.z.max(point.z);
        }

        Some(Cube { min, max })
    }

    pub fn difference(&self, other: &Self) -> Self {
        Self::from(
            self.points
                .difference(&other.points)
                .cloned()
                .collect::<HashSet<_>>(),
        )
    }

    pub fn explore(&self, start: Point) -> Option<Self> {
        if !self.points.contains(&start) {
            return None;
        }

        let mut frontier = VecDeque::new();
        let mut reached = HashSet::new();

        frontier.push_back(start);
        reached.insert(start);

        while let Some(current) = frontier.pop_front() {
            for next in self.neighbors(&current) {
                if !reached.contains(&next) {
                    frontier.push_back(next);
                    reached.insert(next);
                }
            }
        }

        Some(Self { points: reached })
    }
}

impl From<HashSet<Point>> for CubeGrid {
    fn from(points: HashSet<Point>) -> Self {
        Self { points }
    }
}

impl From<Vec<Point>> for CubeGrid {
    fn from(points: Vec<Point>) -> Self {
        Self {
            points: points.into_iter().collect(),
        }
    }
}
