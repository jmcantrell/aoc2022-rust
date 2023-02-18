pub type Direction = nalgebra::Vector3<isize>;

pub const X_POSITIVE: Direction = Direction::new(1, 0, 0);
pub const X_NEGATIVE: Direction = Direction::new(-1, 0, 0);
pub const Y_POSITIVE: Direction = Direction::new(0, 1, 0);
pub const Y_NEGATIVE: Direction = Direction::new(0, -1, 0);
pub const Z_POSITIVE: Direction = Direction::new(0, 0, 1);
pub const Z_NEGATIVE: Direction = Direction::new(0, 0, -1);

pub const DIRECTIONS: [Direction; 6] = [
    X_POSITIVE, X_NEGATIVE, Y_POSITIVE, Y_NEGATIVE, Z_POSITIVE, Z_NEGATIVE,
];
