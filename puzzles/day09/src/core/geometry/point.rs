use std::ops::{Add, AddAssign, Sub};

use super::Direction;

pub const ORIGIN: Point = Point { x: 0, y: 0 };

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn abs(self) -> Point {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn unit(self) -> Point {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}

impl Add<Self> for Point {
    type Output = Self;

    fn add(self, other: Point) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Self> for Point {
    type Output = Self;

    fn sub(self, other: Point) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl AddAssign<Self> for Point {
    fn add_assign(&mut self, other: Point) {
        *self = *self + other;
    }
}

impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        self + direction.offset()
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, direction: Direction) {
        *self = *self + direction;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ORIGIN: Point = Point { x: 0, y: 0 };

    #[test]
    fn add() {
        let point = Point { x: 1, y: 2 };
        assert_eq!(ORIGIN + point, point);
    }

    #[test]
    fn add_assign() {
        let point1 = Point { x: 1, y: 2 };
        let point2 = Point { x: 5, y: 10 };

        let mut p = point1;
        p += point2;

        assert_eq!(p, point1 + point2);
    }

    #[test]
    fn sub() {
        assert_eq!(ORIGIN - Point { x: 1, y: 2 }, Point { x: -1, y: -2 });
    }

    #[test]
    fn add_direction() {
        assert_eq!(ORIGIN + Direction::Up, Point { x: 0, y: 1 });
        assert_eq!(ORIGIN + Direction::Down, Point { x: 0, y: -1 });
        assert_eq!(ORIGIN + Direction::Left, Point { x: -1, y: 0 });
        assert_eq!(ORIGIN + Direction::Right, Point { x: 1, y: 0 });
    }

    #[test]
    fn add_assign_direction() {
        let mut point = ORIGIN;

        point += Direction::Up;
        assert_eq!(point, Point { x: 0, y: 1 });

        point += Direction::Right;
        assert_eq!(point, Point { x: 1, y: 1 });

        point += Direction::Down;
        assert_eq!(point, Point { x: 1, y: 0 });

        point += Direction::Left;
        assert_eq!(point, ORIGIN);
    }
}
