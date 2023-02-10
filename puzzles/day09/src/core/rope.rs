use std::collections::HashSet;

use geometry::{Location, RelativeDirection};

use super::Movement;

use RelativeDirection::*;

type Point = Location<isize>;

#[derive(Debug)]
pub struct Rope {
    pub knots: Vec<Point>,
    pub trail: HashSet<Point>,
}

fn rotate(point: Point, direction: &RelativeDirection) -> Point {
    let (x, y) = match direction {
        Up => (0, 1),
        Down => (0, -1),
        Left => (-1, 0),
        Right => (1, 0),
    };

    Point::new(point.x + x, point.y + y)
}

fn adjust_tail(head: Point, mut tail: Point) -> Point {
    let diff = head - tail;
    let dist = diff.abs();

    if dist.x > 1 || dist.y > 1 {
        tail += (head - tail).signum();
    }

    tail
}

impl Rope {
    pub fn new(size: usize) -> Self {
        let knots = (0..size).map(|_| Point::default()).collect();

        let mut trail = HashSet::new();

        if size > 1 {
            trail.insert(Point::default());
        }

        Self { knots, trail }
    }

    pub fn drag_step(&mut self, direction: &RelativeDirection) {
        if self.knots.is_empty() {
            return;
        }

        self.knots[0] = rotate(self.knots[0], direction);

        let mut a = self.knots[0];

        for b in self.knots.iter_mut().skip(1) {
            *b = adjust_tail(a, *b);
            a = *b;
        }
        self.trail.insert(a);
    }

    pub fn drag(&mut self, movement: &Movement) {
        for _ in 0..movement.magnitude {
            self.drag_step(&movement.direction);
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;

    use super::*;

    fn parse_direction(c: char) -> anyhow::Result<RelativeDirection> {
        use RelativeDirection::*;
        match c {
            'U' => Ok(Up),
            'D' => Ok(Down),
            'L' => Ok(Left),
            'R' => Ok(Right),
            _ => Err(anyhow!("invalid direction: {:?}", c)),
        }
    }

    fn assert_tail_after(expected_tail: Point, path: &str) {
        let directions = path
            .chars()
            .map(parse_direction)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let tail = Point::default();
        let mut head = tail;

        for direction in directions.iter() {
            head = rotate(head, direction);
        }

        assert_eq!(adjust_tail(head, tail), expected_tail);
    }

    macro_rules! assert_tail {
        ($tail:expr, $($paths:expr),+) => {
            for path in [$($paths),+] {
                assert_tail_after($tail.into(), path);
            }
        };
    }

    #[test]
    fn no_tail_move() {
        assert_tail!(
            (0, 0),
            "U",
            "UL",
            "UR",
            "D",
            "DL",
            "DR",
            "L",
            "LU",
            "LD",
            "R",
            "RU",
            "RD"
        );
    }

    #[test]
    fn vertical_tail_move() {
        assert_tail!((0, 1), "UU");
        assert_tail!((0, -1), "DD");
    }

    #[test]
    fn horizontal_tail_move() {
        assert_tail!((-1, 0), "LL");
        assert_tail!((1, 0), "RR");
    }

    #[test]
    fn diagonal_tail_move() {
        assert_tail!((-1, -1), "DLD", "LDL", "LDD", "DLL");
        assert_tail!((1, -1), "DRD", "RDR", "RDD", "DRR");
        assert_tail!((-1, 1), "ULU", "LUL", "LUU", "ULL");
        assert_tail!((1, 1), "URU", "RUR", "RUU", "URR")
    }
}
