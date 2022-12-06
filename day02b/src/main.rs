use anyhow::Context;
use std::convert::TryFrom;
use std::fs;

type Key = char;
type Score = usize;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn that_beats(shape: &Self) -> Self {
        match shape {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn that_is_beaten_by(shape: &Self) -> Self {
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
            Some(*self == Shape::that_beats(other))
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

impl TryFrom<Key> for Shape {
    type Error = anyhow::Error;

    fn try_from(key: Key) -> Result<Self, Self::Error> {
        match key {
            'A' => Ok(Self::Rock),
            'B' => Ok(Self::Paper),
            'C' => Ok(Self::Scissors),
            _ => {
                anyhow::bail!("Invalid shape key {:?}", key);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn determine(shape1: &Shape, shape2: &Shape) -> (Self, (Score, Score)) {
        let score1 = shape1.score();
        let score2 = shape2.score();

        match shape1.beats(shape2) {
            Some(true) => (Self::Win, (score1 + 6, score2)),
            Some(false) => (Self::Lose, (score1, score2 + 6)),
            None => (Self::Draw, (score1 + 3, score2 + 3)),
        }
    }

    fn predict(&self, shape: &Shape) -> Shape {
        match self {
            Outcome::Draw => shape.clone(),
            Outcome::Win => Shape::that_beats(shape),
            Outcome::Lose => Shape::that_is_beaten_by(shape),
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Outcome::Win => Outcome::Lose,
            Outcome::Lose => Outcome::Win,
            Outcome::Draw => Outcome::Draw,
        }
    }
}

impl TryFrom<Key> for Outcome {
    type Error = anyhow::Error;

    fn try_from(key: Key) -> Result<Self, Self::Error> {
        match key {
            'X' => Ok(Self::Lose),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => {
                anyhow::bail!("Invalid outcome key {:?}", key);
            }
        }
    }
}

fn parse_plays(s: &str) -> anyhow::Result<Vec<(Shape, Outcome)>> {
    Ok(s.lines()
        .enumerate()
        .map(|(line, s)| -> anyhow::Result<(Shape, Outcome)> {
            let mut keys = s.split_whitespace();

            let shape_key = keys
                .next()
                .with_context(|| format!("Missing player 1 shape key on line {}", line + 1))?
                .chars()
                .next()
                .unwrap();

            let shape1: Shape = shape_key
                .try_into()
                .with_context(|| format!("Unable to get player 1 shape on line {}", line + 1))?;

            let outcome_key = keys
                .next()
                .with_context(|| format!("Missing outcome key on line {}", line + 1))?
                .chars()
                .next()
                .unwrap();

            let my_outcome: Outcome = outcome_key
                .try_into()
                .with_context(|| format!("Unable to get outcome on line {}", line + 1,))?;

            Ok((shape1, my_outcome))
        })
        .collect::<Result<Vec<_>, _>>()?)
}

fn play_games(plays: &Vec<(Shape, Outcome)>) -> Vec<(Score, Score)> {
    plays
        .iter()
        .map(|(shape1, my_outcome)| {
            let shape2 = my_outcome.predict(shape1);
            let (their_outcome, round) = Outcome::determine(shape1, &shape2);
            assert_eq!(my_outcome, &their_outcome.opposite());
            round
        })
        .collect()
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let plays = parse_plays(&input)?;
    let scores = play_games(&plays);

    let my_total_score: Score = scores.iter().map(|(_, score2)| score2).sum();

    dbg!(&my_total_score);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let input = "A Y\nB X\nC Z";

        let plays = parse_plays(&input)?;
        let scores = play_games(&plays);

        let my_total_score: Score = scores.iter().map(|(_, score2)| score2).sum();

        assert_eq!(my_total_score, 12);

        Ok(())
    }
}
