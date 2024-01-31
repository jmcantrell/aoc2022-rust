use aoc::{Input, Parse, Solve};

use day17::aoc::{Parser1, Parser2, Solver1, Solver2};

const INPUT: Input = include_str!("../input");

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
