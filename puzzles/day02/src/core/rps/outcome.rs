use super::Shape;

pub const OUTCOMES: [Outcome; 3] = [Outcome::Lose, Outcome::Draw, Outcome::Win];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    pub fn ensure(&self, against: &Shape) -> Shape {
        match self {
            Outcome::Draw => against.clone(),
            Outcome::Win => Shape::that_beats(against),
            Outcome::Lose => Shape::that_is_beaten_by(against),
        }
    }
}
