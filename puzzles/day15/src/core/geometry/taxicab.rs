use std::ops::RangeInclusive;

use super::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaxicabCircle {
    pub center: Point,
    pub radius: isize,
}

pub fn manhattan_distance(a: &Point, b: &Point) -> isize {
    (a.x.abs_diff(b.x) + a.y.abs_diff(b.y)).min(isize::MAX as usize) as isize
}

impl TaxicabCircle {
    pub fn from_points(center: Point, other: &Point) -> Self {
        Self {
            center,
            radius: manhattan_distance(&center, other),
        }
    }

    pub fn top(&self) -> isize {
        self.center.y - self.radius
    }

    pub fn bottom(&self) -> isize {
        self.center.y + self.radius
    }

    pub fn contains(&self, point: &Point) -> bool {
        manhattan_distance(&self.center, point) <= self.radius
    }

    pub fn contains_y(&self, y: isize) -> bool {
        self.top() <= y && y <= self.bottom()
    }

    pub fn x_range(&self, y: isize) -> Option<RangeInclusive<isize>> {
        if self.contains_y(y) {
            let offset = self.radius - (self.center.y - y).abs();
            Some((self.center.x - offset)..=(self.center.x + offset))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CENTER: Point = Point { x: 0, y: 0 };

    #[test]
    fn manhattan_distance_perpendicular() {
        let max_dist: isize = 10;

        for i in -max_dist..=max_dist {
            assert_eq!(
                super::manhattan_distance(&CENTER, &Point { x: i, y: 0 }),
                i.abs()
            );
            assert_eq!(
                super::manhattan_distance(&CENTER, &Point { x: 0, y: i }),
                i.abs()
            );
        }
    }

    #[test]
    fn manhattan_distance_diagonal() {
        let dist: isize = 10;

        let half = dist / 2;

        assert_eq!(
            super::manhattan_distance(&CENTER, &Point { x: half, y: half }),
            dist.abs()
        );

        assert_eq!(
            super::manhattan_distance(&CENTER, &Point { x: -half, y: half }),
            dist.abs()
        );

        assert_eq!(
            super::manhattan_distance(&CENTER, &Point { x: half, y: -half }),
            dist.abs()
        );

        assert_eq!(
            super::manhattan_distance(&CENTER, &Point { x: -half, y: -half }),
            dist.abs()
        );
    }
}
