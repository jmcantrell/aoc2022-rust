use super::Point;

pub const X_POSITIVE: Point = Point { x: 1, y: 0, z: 0 };
pub const X_NEGATIVE: Point = Point { x: -1, y: 0, z: 0 };
pub const Y_POSITIVE: Point = Point { x: 0, y: 1, z: 0 };
pub const Y_NEGATIVE: Point = Point { x: 0, y: -1, z: 0 };
pub const Z_POSITIVE: Point = Point { x: 0, y: 0, z: 1 };
pub const Z_NEGATIVE: Point = Point { x: 0, y: 0, z: -1 };

pub const DIRECTIONS: [Point; 6] = [
    X_POSITIVE, X_NEGATIVE, Y_POSITIVE, Y_NEGATIVE, Z_POSITIVE, Z_NEGATIVE,
];
