use crate::core::Machine;

use super::{Parsed1, Parsed2};

pub type Solution1 = isize;
pub type Solution2 = String;

pub fn solve1(program: &Parsed1) -> anyhow::Result<Solution1> {
    let start = 20;
    let step = 40;

    let mut machine = Machine::new(program.clone());

    let signal_strengths = machine
        .run()
        .enumerate()
        .skip(start - 1)
        .step_by(step)
        .map(|(i, register)| (i as isize + 1) * register);

    Ok(signal_strengths.sum())
}

pub fn solve2(program: &Parsed2) -> anyhow::Result<Solution2> {
    let mut display = String::new();
    let mut machine = Machine::new(program.clone());

    let width = 40;

    for (i, register) in machine.run().enumerate() {
        let position = i % width;

        let mut lit = false;

        if register + 1 >= 0 {
            let start = (register - 1).max(0) as usize;
            let end = (register + 1) as usize;

            lit = start <= position && position <= end;
        }

        display += if lit { "#" } else { "." };

        if (i + 1) % width == 0 {
            display += "\n";
        }
    }

    Ok(display)
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");
    const OUTPUT2: Input = include_str!("../../output-test-2.txt");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 13140);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, OUTPUT2);
        Ok(())
    }
}
