use super::Shape;

use Outcome::*;

pub const OUTCOMES: [Outcome; 3] = [Lose, Draw, Win];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    pub fn ensure(&self, against: &Shape) -> Shape {
        match self {
            Draw => *against,
            Win => Shape::that_beats(against),
            Lose => Shape::that_is_beaten_by(against),
        }
    }
}
