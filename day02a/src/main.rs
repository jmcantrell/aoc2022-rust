use anyhow::Context;
use std::fs;

type Key = char;
type Score = usize;
type Round = (Score, Score);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> Score {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn beats(&self, other: &Self) -> Option<bool> {
        if self == other {
            None
        } else {
            match self {
                Self::Rock => Some(*other == Self::Scissors),
                Self::Paper => Some(*other == Self::Rock),
                Self::Scissors => Some(*other == Self::Paper),
            }
        }
    }
}

#[derive(Debug)]
struct Player {
    rock: Key,
    paper: Key,
    scissors: Key,
}

impl Player {
    fn new(rock: Key, paper: Key, scissors: Key) -> Self {
        Self {
            rock,
            paper,
            scissors,
        }
    }

    fn get_shape(&self, key: Key) -> anyhow::Result<Shape> {
        if self.rock == key {
            return Ok(Shape::Rock);
        } else if self.paper == key {
            return Ok(Shape::Paper);
        } else if self.scissors == key {
            return Ok(Shape::Scissors);
        } else {
            anyhow::bail!("Invalid shape key {:?}", key);
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let player1 = Player::new('A', 'B', 'C');
    let player2 = Player::new('X', 'Y', 'Z');

    let rounds = input
        .lines()
        .enumerate()
        .map(|(line, s)| -> anyhow::Result<Round> {
            let keys: Vec<_> = s.split_whitespace().collect();

            if keys.len() != 2 {
                anyhow::bail!(
                    "Line {} has unexpected number of words: {} != 2",
                    line,
                    keys.len()
                );
            }

            let mut keys = keys.iter();

            let shape1: Shape =
                player1.get_shape(keys.next().unwrap().chars().next().with_context(|| {
                    format!("Line {} is missing the player 1 shape key", line)
                })?)?;

            let shape2: Shape =
                player2.get_shape(keys.next().unwrap().chars().next().with_context(|| {
                    format!("Line {} is missing the player 2 shape key", line)
                })?)?;

            let score1 = shape1.score();
            let score2 = shape2.score();

            match shape1.beats(&shape2) {
                Some(true) => Ok((score1 + 6, score2)),
                Some(false) => Ok((score1, score2 + 6)),
                None => Ok((score1 + 3, score2 + 3)),
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    let my_total_score: Score = rounds.iter().map(|(_, score2)| score2).sum();

    dbg!(&my_total_score);

    Ok(())
}
