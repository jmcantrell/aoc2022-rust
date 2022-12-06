use anyhow::Context;
use std::convert::TryFrom;
use std::fs;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Sections(RangeInclusive<usize>);

impl Sections {
    fn contains(&self, other: &Self) -> bool {
        self.0.start() <= other.0.start() && other.0.end() <= self.0.end()
    }
}

impl TryFrom<&str> for Sections {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut split = s.splitn(2, '-');

        let a: usize = split
            .next()
            .context("Missing sections left-hand side")?
            .parse()?;

        let b: usize = split
            .next()
            .context("Missing sections right-hand side")?
            .parse()?;

        Ok(Self(a..=b))
    }
}

#[derive(Debug)]
struct Pair(Sections, Sections);

impl Pair {
    fn has_redundancy(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }
}

impl TryFrom<&str> for Pair {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut split = s.splitn(2, ',');

        let a: Sections = split
            .next()
            .context("Missing first sections range")?
            .try_into()?;

        let b: Sections = split
            .next()
            .context("Missing second sections range")?
            .try_into()?;

        Ok(Self(a, b))
    }
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let pairs = input
        .lines()
        .enumerate()
        .map(|(line, s)| -> anyhow::Result<Pair> {
            Ok(s.try_into()
                .with_context(|| format!("Unable to parse pair on line {}", line))?)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let num_pairs_with_redundancy = pairs.into_iter().filter(Pair::has_redundancy).count();

    dbg!(num_pairs_with_redundancy);

    Ok(())
}
