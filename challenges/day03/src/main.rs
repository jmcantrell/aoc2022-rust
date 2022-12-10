extern crate day03;

use aoc::{Parser, Solver};
use day03::{
    parser::{Parser1, Parser2},
    solver::{Solver1, Solver2},
};

const INPUT: &'static str = include_str!("../input.txt");

fn main() -> anyhow::Result<()> {
    println!(
        "Part 1 solution: {:?}",
        Solver1(Parser1(INPUT).parse()?).solve()?
    );
    println!(
        "Part 2 solution: {:?}",
        Solver2(Parser2(INPUT).parse()?).solve()?
    );
    Ok(())
}
