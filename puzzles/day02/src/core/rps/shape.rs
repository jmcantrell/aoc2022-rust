use super::{Outcome, Score};

use Shape::*;
pub const SHAPES: [Shape; 3] = [Rock, Paper, Scissors];

use Outcome::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn score(&self) -> Score {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    pub fn that_beats(shape: &Self) -> Self {
        match shape {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    pub fn that_is_beaten_by(shape: &Self) -> Self {
        match shape {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    pub fn against(&self, other: &Self) -> Outcome {
        if self != other {
            if *self == Shape::that_beats(other) {
                Win
            } else {
                Lose
            }
        } else {
            Draw
        }
    }
}
