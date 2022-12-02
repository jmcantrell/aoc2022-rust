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

    fn get_shape(&self, key: Key) -> Result<Shape, Key> {
        if self.rock == key {
            return Ok(Shape::Rock);
        } else if self.paper == key {
            return Ok(Shape::Paper);
        } else if self.scissors == key {
            return Ok(Shape::Scissors);
        } else {
            return Err(key);
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let player1 = Player::new('A', 'B', 'C');
    let player2 = Player::new('X', 'Y', 'Z');

    let rounds: Vec<Round> = input
        .lines()
        .map(|line| {
            let mut keys = line.split_whitespace().map(|s| s.chars().next().unwrap());

            let shape1 = player1.get_shape(keys.next().unwrap()).unwrap();
            let shape2 = player2.get_shape(keys.next().unwrap()).unwrap();

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
