use std::collections::HashSet;

use super::{Parsed1, Parsed2};

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(graph: &Parsed1) -> anyhow::Result<Solution1> {
    let seconds = 30;
    let mut max_released = 0;
    let all_valves: HashSet<_> = graph.flow_rates.keys().cloned().collect();

    graph.traverse_possible_paths(&all_valves, seconds, |_, _, released| {
        max_released = max_released.max(released);
    });

    Ok(max_released)
}

pub fn solve2(graph: &Parsed2) -> anyhow::Result<Solution2> {
    let seconds = 26;
    let mut max_released = 0;
    let all_valves: HashSet<_> = graph.flow_rates.keys().cloned().collect();

    graph.traverse_possible_paths(
        &all_valves,
        seconds,
        |_, remaining_valves, released_by_me| {
            graph.traverse_possible_paths(
                &remaining_valves,
                seconds,
                |_, _, released_by_elephant| {
                    max_released = max_released.max(released_by_me + released_by_elephant);
                },
            );
        },
    );

    Ok(max_released)
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 1651);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 1707);
        Ok(())
    }
}
