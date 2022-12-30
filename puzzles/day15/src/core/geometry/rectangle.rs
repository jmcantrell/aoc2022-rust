use super::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Rectangle {
    pub fn points(&self) -> impl Iterator<Item = Point> + '_ {
        (self.top_left.y..=self.bottom_right.y)
            .map(move |y| (self.top_left.x..=self.bottom_right.x).map(move |x| Point { x, y }))
            .flatten()
    }

    pub fn top(&self) -> isize {
        self.top_left.y
    }

    pub fn left(&self) -> isize {
        self.top_left.x
    }

    pub fn bottom(&self) -> isize {
        self.bottom_right.y
    }

    pub fn right(&self) -> isize {
        self.bottom_right.x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points() {
        assert_eq!(
            Rectangle {
                top_left: Point { x: -1, y: -1 },
                bottom_right: Point { x: 1, y: 1 },
            }
            .points()
            .collect::<Vec<_>>(),
            vec![
                Point::new(-1, -1),
                Point::new(0, -1),
                Point::new(1, -1),
                Point::new(-1, 0),
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(-1, 1),
                Point::new(0, 1),
                Point::new(1, 1),
            ]
        )
    }
}
