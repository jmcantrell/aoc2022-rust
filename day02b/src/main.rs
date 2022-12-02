use std::convert::TryFrom;
use std::fs;

type Key = char;
type Score = usize;
type Round = (Score, Score);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<Key> for Shape {
    type Error = Key;

    fn try_from(c: Key) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Self::Rock),
            'B' => Ok(Self::Paper),
            'C' => Ok(Self::Scissors),
            _ => Err(c),
        }
    }
}

impl Shape {
    fn that_beats(shape: Self) -> Self {
        match shape {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn that_is_beaten_by(shape: Self) -> Self {
        match shape {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn beats(&self, other: &Self) -> Option<bool> {
        if self == other {
            None
        } else {
            Some(*self == Shape::that_beats(*other))
        }
    }

    fn score(&self) -> Score {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl TryFrom<Key> for Outcome {
    type Error = Key;

    fn try_from(c: Key) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(Self::Lose),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(c),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let rounds: Vec<Round> = input
        .lines()
        .map(|line| {
            let mut keys = line.split_whitespace().map(|s| s.chars().next().unwrap());

            let shape1: Shape = keys.next().unwrap().try_into().unwrap();
            let outcome: Outcome = keys.next().unwrap().try_into().unwrap();
            let shape2 = match outcome {
                Outcome::Draw => shape1,
                Outcome::Win => Shape::that_beats(shape1),
                Outcome::Lose => Shape::that_is_beaten_by(shape1),
            };

            let score1 = shape1.score();
            let score2 = shape2.score();

            match shape1.beats(&shape2) {
                Some(true) => (score1 + 6, score2),
                Some(false) => (score1, score2 + 6),
                None => (score1 + 3, score2 + 3),
            }
        })
        .collect();

    let my_total_score: Score = rounds.iter().map(|(_, score2)| score2).sum();

    dbg!(&my_total_score);
}
