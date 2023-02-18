use aoc::{Input, Parse, Solve};

use day07::aoc::{Parser, Solver1, Solver2};

const INPUT: Input = include_str!("../input.txt");

fn main() -> anyhow::Result<()> {
    println!(
        "Part 1 solution: {:?}",
        Solver1(Parser(INPUT).parse()?).solve()?
    );
    println!(
        "Part 2 solution: {:?}",
        Solver2(Parser(INPUT).parse()?).solve()?
    );
    Ok(())
}
