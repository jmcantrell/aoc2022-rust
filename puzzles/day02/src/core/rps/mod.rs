pub mod outcome;
pub use outcome::*;

pub mod shape;
pub use shape::*;

pub type Score = usize;
pub type Round = (Outcome, (Score, Score));

pub fn play(shape1: &Shape, shape2: &Shape) -> Round {
    let score1 = shape1.score();
    let score2 = shape2.score();

    let outcome = shape1.against(shape2);

    let scores = match outcome {
        Outcome::Win => (score1 + 6, score2),
        Outcome::Lose => (score1, score2 + 6),
        Outcome::Draw => (score1 + 3, score2 + 3),
    };

    (outcome, scores)
}
