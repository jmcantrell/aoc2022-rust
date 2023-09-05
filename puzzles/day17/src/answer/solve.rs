use crate::core::{Chamber, SampleKey};
use std::collections::HashMap;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(jet_pattern: &Parsed1) -> anyhow::Result<Solution1> {
    let mut chamber = Chamber::new(jet_pattern);

    for _ in 0..2022 {
        chamber.drop_rock();
    }

    Ok(chamber.height())
}

pub fn solve2(jet_pattern: &Parsed2) -> anyhow::Result<Solution2> {
    type Iteration = usize;
    type Height = usize;

    let num_rocks = 1_000_000_000_000;

    let mut chamber = Chamber::new(jet_pattern);
    let mut iterations: HashMap<SampleKey, Vec<(Iteration, Height)>> = Default::default();

    for i in 0..num_rocks {
        let sample_key = chamber.drop_rock();
        let samples = iterations.entry(sample_key).or_default();

        samples.push((i, chamber.height()));

        if samples.len() > 1 {
            let (prev_iteration, prev_height) = samples[0];
            let (next_iteration, next_height) = samples[1];

            let rocks_added = next_iteration - prev_iteration;
            let height_added = next_height - prev_height;

            let (num_cycles, rock_num) = (num_rocks / rocks_added, num_rocks % rocks_added);

            for samples in iterations.values() {
                for &(i, height) in samples.iter() {
                    if i + 1 == rock_num {
                        return Ok(num_cycles * height_added + height);
                    }
                }
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 3_068);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 1_514_285_714_288);
        Ok(())
    }
}
