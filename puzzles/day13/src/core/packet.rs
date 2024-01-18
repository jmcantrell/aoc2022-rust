use std::cmp::Ordering;
use std::convert::TryFrom;

use anyhow::{anyhow, ensure, Context};

pub type PacketPair = (Packet, Packet);

#[derive(Debug, Clone)]
pub enum Packet {
    Integer(usize),
    List(Vec<Self>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(list1), Packet::List(list2)) => {
                let maybe_diff = list1
                    .iter()
                    .zip(list2.iter())
                    .find(|(packet1, packet2)| packet1 != packet2);

                if let Some((packet1, packet2)) = maybe_diff {
                    packet1.cmp(packet2)
                } else {
                    list1.len().cmp(&list2.len())
                }
            }
            (Packet::Integer(value1), Packet::Integer(value2)) => value1.cmp(value2),
            (Packet::Integer(_), Packet::List(_)) => Packet::List(vec![self.clone()]).cmp(other),
            (Packet::List(_), Packet::Integer(_)) => self.cmp(&Packet::List(vec![other.clone()])),
        }
    }
}

impl TryFrom<&str> for Packet {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        fn parse_int(s: &str) -> anyhow::Result<(Option<Packet>, &str)> {
            if s.starts_with(|c: char| c.is_ascii_digit()) {
                let i = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
                let (prefix, rest) = s.split_at(i);
                let value: usize = prefix.parse().unwrap();
                Ok((Some(Packet::Integer(value)), rest))
            } else {
                Ok((None, s))
            }
        }

        fn parse_list(s: &str) -> anyhow::Result<(Option<Packet>, &str)> {
            if let Some(mut s) = s.strip_prefix('[') {
                let mut packets: Vec<Packet> = Default::default();
                let mut comma = false;

                loop {
                    let (maybe_packet, mut rest) = parse_packet(s)?;

                    if let Some(packet) = maybe_packet {
                        packets.push(packet);
                        if let Some(suffix) = rest.strip_prefix(',') {
                            comma = true;
                            rest = suffix;
                        } else {
                            comma = false;
                        }
                    } else {
                        if comma {
                            return Err(anyhow!("trailing comma before: {:?}", rest));
                        }
                        break;
                    }

                    s = rest;
                }

                let s = s
                    .strip_prefix(']')
                    .with_context(|| format!("missing list closing character before: {s:?}"))?;

                Ok((Some(Packet::List(packets)), s))
            } else {
                Ok((None, s))
            }
        }

        fn parse_packet(s: &str) -> anyhow::Result<(Option<Packet>, &str)> {
            for parser in [parse_int, parse_list] {
                let (maybe_packet, s) = parser(s)?;
                if let Some(packet) = maybe_packet {
                    return Ok((Some(packet), s));
                }
            }

            Ok((None, s))
        }

        let (maybe_packet, rest) = parse_packet(s)?;
        ensure!(rest.is_empty(), "leftover characters: {rest:?}");
        maybe_packet.with_context(|| format!("invalid packet: {s:?}"))
    }
}

#[macro_export]
macro_rules! packet {
    ($n:literal) => {
        $crate::core::Packet::Integer($n)
    };
    ([$($i:tt),*]) => {
        $crate::core::Packet::List(vec![
            $($crate::packet!($i)),*
        ])
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_packet_int_ord() {
        assert!(packet!(0) < packet!(1));
        assert!(packet!(1) == packet!(1));
        assert!(packet!(1) > packet!(0));
    }

    #[test]
    fn test_packet_list_ord() {
        assert!(packet!([]) == packet!([]));

        assert!(packet!([]) < packet!([0]));
        assert!(packet!([0]) > packet!([]));

        assert!(packet!([0]) < packet!([1]));
        assert!(packet!([1]) == packet!([1]));
        assert!(packet!([1]) > packet!([0]));
    }

    #[test]
    fn test_packet_int_list_ord() {
        assert!(packet!(0) == packet!([0]));
        assert!(packet!([0]) == packet!(0));
    }
}
