use anyhow::Context;

use aoc::Input;

use crate::core::{Packet, PacketPair};

type Parsed = Vec<PacketPair>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    fn parse_packet_pair(s: &str) -> anyhow::Result<PacketPair> {
        let packets = s
            .lines()
            .enumerate()
            .map(|(i, s)| Packet::try_from(s).with_context(|| format!("packet number {}", i + 1)))
            .collect::<Result<Vec<_>, _>>()?;

        let mut packets = packets.into_iter();

        let packet1 = packets.next().context("missing packet number 1")?;
        let packet2 = packets.next().context("missing packet number 2")?;

        Ok((packet1, packet2))
    }

    input
        .split("\n\n")
        .enumerate()
        .map(|(i, s)| parse_packet_pair(s).with_context(|| format!("pair number {}", i + 1)))
        .collect::<Result<Vec<_>, _>>()
}

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    parse(input)
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    parse(input)
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use super::*;

    const INPUT: Input = include_str!("../../input-test");

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        dbg!(parse(INPUT)?);
        Ok(())
    }
}
