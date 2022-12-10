use anyhow::Context;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct Assignment(RangeInclusive<usize>);

impl Assignment {
    pub fn contains(&self, other: &Self) -> bool {
        self.0.start() <= other.0.start() && other.0.end() <= self.0.end()
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        other.0.contains(self.0.start()) || other.0.contains(self.0.end())
    }
}

impl TryFrom<&str> for Assignment {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut split = s.splitn(2, '-');

        let parse_int = |s: &str| {
            s.parse()
                .with_context(|| format!("invalid integer: {:?}", s))
        };

        let a: usize = parse_int(split.next().context("missing left-hand side")?)?;
        let b: usize = parse_int(split.next().context("missing left-hand side")?)?;

        Ok(Self(a..=b))
    }
}

#[derive(Debug, Clone)]
pub struct AssignmentPair(Assignment, Assignment);

impl AssignmentPair {
    pub fn has_redundancy(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    pub fn has_overlap(&self) -> bool {
        self.0.overlaps(&self.1) || self.1.overlaps(&self.0)
    }
}

impl TryFrom<&str> for AssignmentPair {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut items = s.splitn(2, ',');

        let parse_assignment = |s: &str| {
            Assignment::try_from(s).with_context(|| format!("invalid assignment: {:?}", s))
        };

        let a = parse_assignment(items.next().context("missing first assignment")?)?;
        let b = parse_assignment(items.next().context("missing second assignment")?)?;

        Ok(Self(a, b))
    }
}
