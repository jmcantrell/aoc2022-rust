use std::collections::HashSet;

use crate::core::{Direction, Point, Vector, ORIGIN};

#[derive(Debug)]
pub struct Rope {
    pub knots: Vec<Point>,
    pub trail: HashSet<Point>,
}

fn adjust_tail(head: Point, mut tail: Point) -> Point {
    let diff = head - tail;
    let dist = diff.abs();

    if dist.x > 1 || dist.y > 1 {
        tail += (head - tail).unit();
    }

    tail
}

impl Rope {
    pub fn new(size: usize) -> Self {
        let knots = (0..size).map(|_| ORIGIN).collect();

        let mut trail = HashSet::new();

        if size > 1 {
            trail.insert(ORIGIN);
        }

        Self { knots, trail }
    }

    pub fn drag_step(&mut self, direction: &Direction) {
        if self.knots.is_empty() {
            return;
        }

        self.knots[0] += *direction;

        let mut a = self.knots[0];

        for b in self.knots.iter_mut().skip(1) {
            *b = adjust_tail(a, *b);
            a = *b;
        }
        self.trail.insert(a);
    }

    pub fn drag(&mut self, vector: &Vector) {
        for _ in 0..vector.magnitude {
            self.drag_step(&vector.direction);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_tail_after(expected_tail: Point, path: &str) {
        let directions = path
            .chars()
            .map(Direction::try_from)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let tail = ORIGIN;
        let mut head = tail;

        for direction in directions.iter() {
            head += *direction;
        }

        assert_eq!(adjust_tail(head, tail), expected_tail);
    }

    macro_rules! assert_tail {
        ($tail:expr, $($paths:expr),+) => {
            for path in vec![$($paths),+] {
                assert_tail_after($tail, path);
            }
        };
    }

    #[test]
    fn no_tail_move() {
        assert_tail!(ORIGIN, "U", "UL", "UR", "D", "DL", "DR", "L", "LU", "LD", "R", "RU", "RD");
    }

    #[test]
    fn vertical_tail_move() {
        assert_tail!(Point::new(0, 1), "UU");
        assert_tail!(Point::new(0, -1), "DD");
    }

    #[test]
    fn horizontal_tail_move() {
        assert_tail!(Point::new(-1, 0), "LL");
        assert_tail!(Point::new(1, 0), "RR");
    }

    #[test]
    fn diagonal_tail_move() {
        assert_tail!(Point::new(-1, -1), "DLD", "LDL", "LDD", "DLL");
        assert_tail!(Point::new(1, -1), "DRD", "RDR", "RDD", "DRR");
        assert_tail!(Point::new(-1, 1), "ULU", "LUL", "LUU", "ULL");
        assert_tail!(Point::new(1, 1), "URU", "RUR", "RUU", "URR")
    }
}
