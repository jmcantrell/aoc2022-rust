use crate::rps::{Outcome, Shape};
use anyhow::{anyhow, Context};
use aoc::Parser;
use std::ops::RangeInclusive;

const SHAPES: [Shape; 3] = [Shape::Rock, Shape::Paper, Shape::Scissors];
const OUTCOMES: [Outcome; 3] = [Outcome::Lose, Outcome::Draw, Outcome::Win];
const FIELD_RANGES: [RangeInclusive<char>; 2] = ['A'..='C', 'X'..='Z'];

pub type Key = usize;
pub type KeyPair = (Key, Key);
pub type Parsed1 = Vec<(Shape, Shape)>;
pub type Parsed2 = Vec<(Shape, Outcome)>;

fn parse_plays<'i>(input: &'i str) -> anyhow::Result<Vec<KeyPair>> {
    let parse_key = |(i, s): (usize, &str)| -> anyhow::Result<Key> {
        let c = s.chars().next().unwrap();
        if FIELD_RANGES[i].contains(&c) {
            Ok(c as usize - *FIELD_RANGES[i].start() as usize)
        } else {
            Err(anyhow!("invalid integer: {:?}", c))
        }
        .with_context(|| format!("key number {}", i + 1))
    };

    let parse_key_pair = |s: &str| -> anyhow::Result<KeyPair> {
        let mut keys = s.split_whitespace().enumerate().map(parse_key);

        let key1 = keys.next().context("missing key number 1")??;
        let key2 = keys.next().context("missing key number 2")??;

        Ok((key1, key2))
    };

    input
        .lines()
        .enumerate()
        .map(|(i, s)| -> anyhow::Result<KeyPair> {
            parse_key_pair(s).with_context(|| format!("line number {}", i + 1))
        })
        .collect::<Result<Vec<_>, _>>()
        .context("unable to parse plays")
}

#[derive(Debug, Clone)]
pub struct Parser1<'i>(pub &'i str);

impl Parser for Parser1<'_> {
    type Parsed = anyhow::Result<Parsed1>;

    fn parse(&self) -> Self::Parsed {
        Ok(parse_plays(self.0)?
            .into_iter()
            .map(|(key1, key2)| (SHAPES[key1], SHAPES[key2]))
            .collect::<Vec<_>>())
    }
}

#[derive(Debug, Clone)]
pub struct Parser2<'i>(pub &'i str);

impl Parser for Parser2<'_> {
    type Parsed = anyhow::Result<Parsed2>;

    fn parse(&self) -> Self::Parsed {
        Ok(parse_plays(self.0)?
            .into_iter()
            .map(|(key1, key2)| (SHAPES[key1], OUTCOMES[key2]))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../input-test.txt");

    #[test]
    fn parse1() -> anyhow::Result<()> {
        dbg!(Parser1(INPUT).parse()?);
        Ok(())
    }

    #[test]
    fn parse2() -> anyhow::Result<()> {
        dbg!(Parser2(INPUT).parse()?);
        Ok(())
    }
}
