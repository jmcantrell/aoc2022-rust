use anyhow::Context;
use std::fs;

type Key = char;
type Score = usize;

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

fn init_players() -> (Player, Player) {
    (Player::new('A', 'B', 'C'), Player::new('X', 'Y', 'Z'))
}

fn parse_plays(player1: &Player, player2: &Player, s: &str) -> anyhow::Result<Vec<(Shape, Shape)>> {
    Ok(s.lines()
        .enumerate()
        .map(|(line, s)| -> anyhow::Result<(Shape, Shape)> {
            let mut keys = s.split_whitespace();

            let key1 = keys
                .next()
                .with_context(|| format!("Missing player 1 shape key on line {}", line + 1))?
                .chars()
                .next()
                .unwrap();

            let key2 = keys
                .next()
                .with_context(|| format!("Missing player 2 shape key on line {}", line + 1))?
                .chars()
                .next()
                .unwrap();

            let shape1 = player1
                .get_shape(key1)
                .with_context(|| format!("Unable to get player 1 shape on line {}", line + 1))?;

            let shape2 = player2
                .get_shape(key2)
                .with_context(|| format!("Unable to get player 2 shape on line {}", line + 1,))?;

            Ok((shape1, shape2))
        })
        .collect::<Result<Vec<_>, _>>()?)
}

fn play_games(plays: &Vec<(Shape, Shape)>) -> Vec<(Score, Score)> {
    plays
        .iter()
        .map(|(shape1, shape2)| {
            let score1 = shape1.score();
            let score2 = shape2.score();

            match shape1.beats(&shape2) {
                Some(true) => (score1 + 6, score2),
                Some(false) => (score1, score2 + 6),
                None => (score1 + 3, score2 + 3),
            }
        })
        .collect()
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let (player1, player2) = init_players();
    let plays = parse_plays(&player1, &player2, &input)?;
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

        let (player1, player2) = init_players();
        let plays = parse_plays(&player1, &player2, &input)?;
        let scores = play_games(&plays);

        let my_total_score: Score = scores.iter().map(|(_, score2)| score2).sum();

        assert_eq!(my_total_score, 15);

        Ok(())
    }
}
