use std::fs;
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug)]
enum ParseSectionsError {
    NoHyphen,
    InvalidBound(ParseIntError),
}

impl From<ParseIntError> for ParseSectionsError {
    fn from(err: ParseIntError) -> Self {
        Self::InvalidBound(err)
    }
}

#[derive(Debug)]
struct Sections(RangeInclusive<usize>);

impl Sections {
    fn overlaps(&self, other: &Self) -> bool {
        other.0.contains(self.0.start()) || other.0.contains(self.0.end())
    }
}

impl FromStr for Sections {
    type Err = ParseSectionsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains('-') {
            return Err(ParseSectionsError::NoHyphen);
        }

        let mut split = s.splitn(2, '-');

        let a: usize = split.next().unwrap().parse()?;
        let b: usize = split.next().unwrap().parse()?;

        Ok(Self(a..=b))
    }
}

#[derive(Debug)]
enum ParsePairError {
    NoComma,
    InvalidSections(ParseSectionsError),
}

impl From<ParseSectionsError> for ParsePairError {
    fn from(err: ParseSectionsError) -> Self {
        Self::InvalidSections(err)
    }
}

#[derive(Debug)]
struct Pair(Sections, Sections);

impl Pair {
    fn has_overlap(&self) -> bool {
        self.0.overlaps(&self.1) || self.1.overlaps(&self.0)
    }
}

impl FromStr for Pair {
    type Err = ParsePairError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains(',') {
            return Err(ParsePairError::NoComma);
        }

        let mut split = s.splitn(2, ',');

        let a: Sections = split.next().unwrap().parse()?;
        let b: Sections = split.next().unwrap().parse()?;

        Ok(Self(a, b))
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let pairs: Vec<_> = input
        .lines()
        .map(Pair::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let num_pairs_with_redundancy = pairs.into_iter().filter(Pair::has_overlap).count();

    dbg!(num_pairs_with_redundancy);
}
