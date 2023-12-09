use crate::packet;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(packet_pairs: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(packet_pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (packet1, packet2))| {
            if packet1 <= packet2 {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum())
}

pub fn solve2(packet_pairs: &Parsed2) -> anyhow::Result<Solution2> {
    let divider1 = packet!([[2]]);
    let divider2 = packet!([[6]]);

    let mut packets = vec![&divider1, &divider2];

    for (packet1, packet2) in packet_pairs {
        packets.push(packet1);
        packets.push(packet2);
    }

    packets.sort();

    Ok(packets
        .into_iter()
        .enumerate()
        .filter_map(|(i, packet)| {
            if *packet == divider1 || *packet == divider2 {
                Some(i + 1)
            } else {
                None
            }
        })
        .product())
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 13);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 140);
        Ok(())
    }
}
