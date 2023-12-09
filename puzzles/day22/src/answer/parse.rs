use anyhow::Context;

use aoc::Input;

use crate::core::{Board, Cube, Map, Movement};

type Parsed = (Map, Vec<Movement>);
pub type Parsed1 = (Board, Vec<Movement>);
pub type Parsed2 = (Cube, Vec<Movement>);

fn parse(input: Input) -> anyhow::Result<Parsed> {
    fn parse_movements(mut s: &str) -> anyhow::Result<Vec<Movement>> {
        let mut movements = Vec::new();

        while let Some(i) = s.find(|c: char| c == 'L' || c == 'R') {
            movements.push(s[..i].try_into()?); // forward
            movements.push(s[i..=i].try_into()?); // rotate
            s = &s[(i + 1)..];
        }

        movements.push(s.try_into()?); // forward

        Ok(movements)
    }

    let mut chunks = input.splitn(2, "\n\n");

    let map = chunks
        .next()
        .context("missing board")?
        .lines()
        .collect::<Vec<_>>()
        .try_into()?;

    let movements = parse_movements(chunks.next().context("missing movements")?.trim())?;

    Ok((map, movements))
}

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    let (map, path) = parse(input)?;
    Ok((map.into(), path))
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    let (map, path) = parse(input)?;
    Ok((map.try_into()?, path))
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_parse1() -> anyhow::Result<()> {
        dbg!(parse1(INPUT)?);
        Ok(())
    }

    #[test]
    fn test_parse2() -> anyhow::Result<()> {
        dbg!(parse2(INPUT)?);
        Ok(())
    }
}
