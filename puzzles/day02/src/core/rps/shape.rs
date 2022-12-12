use super::{Outcome, Score};

pub const SHAPES: [Shape; 3] = [Shape::Rock, Shape::Paper, Shape::Scissors];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn score(&self) -> Score {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn that_beats(shape: &Self) -> Self {
        match shape {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    pub fn that_is_beaten_by(shape: &Self) -> Self {
        match shape {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    pub fn against(&self, other: &Self) -> Outcome {
        if self != other {
            if *self == Shape::that_beats(other) {
                Outcome::Win
            } else {
                Outcome::Lose
            }
        } else {
            Outcome::Draw
        }
    }
}
