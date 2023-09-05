use std::ops::RangeInclusive;

use anyhow::{anyhow, Context};

use aoc::Input;

use crate::core::{Outcome, Shape, OUTCOMES, SHAPES};
pub type Parsed1 = Vec<(Shape, Shape)>;
pub type Parsed2 = Vec<(Shape, Outcome)>;

type Key = usize;
type KeyPair = (Key, Key);

const FIELD_RANGES: [RangeInclusive<char>; 2] = ['A'..='C', 'X'..='Z'];

fn parse(input: Input) -> anyhow::Result<Vec<KeyPair>> {
    fn parse_key(s: &str, i: usize) -> anyhow::Result<Key> {
        let c = s.chars().next().unwrap();
        let range = &FIELD_RANGES[i];
        if range.contains(&c) {
            Ok(c as usize - *range.start() as usize)
        } else {
            Err(anyhow!("unrecognized key: {:?}", c))
        }
    }

    fn parse_key_pair(s: &str) -> anyhow::Result<KeyPair> {
        let mut keys = s
            .split_whitespace()
            .enumerate()
            .map(|(i, s)| parse_key(s, i).with_context(|| format!("key number {}", i + 1)));

        let key1 = keys.next().context("missing key number 1")??;
        let key2 = keys.next().context("missing key number 2")??;

        Ok((key1, key2))
    }

    fn parse_key_pairs(s: &str) -> anyhow::Result<Vec<KeyPair>> {
        s.lines()
            .enumerate()
            .map(|(i, s)| -> anyhow::Result<KeyPair> {
                parse_key_pair(s).with_context(|| format!("line number {}", i + 1))
            })
            .collect()
    }

    parse_key_pairs(input).context("unable to parse plays")
}

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    Ok(parse(input)?
        .into_iter()
        .map(|(key1, key2)| (SHAPES[key1], SHAPES[key2]))
        .collect::<Vec<_>>())
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    Ok(parse(input)?
        .into_iter()
        .map(|(key1, key2)| (SHAPES[key1], OUTCOMES[key2]))
        .collect())
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn parse1() -> anyhow::Result<()> {
        dbg!(super::parse1(INPUT)?);
        Ok(())
    }

    #[test]
    fn parse2() -> anyhow::Result<()> {
        dbg!(super::parse2(INPUT)?);
        Ok(())
    }
}
