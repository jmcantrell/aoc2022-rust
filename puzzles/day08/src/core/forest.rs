use anyhow::{anyhow, Context};

use geometry::{AxesBounds, Grid, GridLocation};

use super::{ScenicScore, ScenicScores, Visibility};

pub type Height = u8;

#[derive(Debug, Clone)]
pub struct TreePatch {
    pub grid: Grid<Height>,
}

impl TreePatch {
    pub fn visibility(&self) -> impl Iterator<Item = GridLocation> {
        let visibility = Visibility::from(&self.grid);

        visibility
            .grid
            .row_major_locations()
            .filter_map(move |location| visibility.grid.get(&location).unwrap().then_some(location))
    }

    pub fn scenic_scores(&self) -> impl Iterator<Item = (GridLocation, ScenicScore)> {
        let scenic_scores = ScenicScores::from(&self.grid);

        scenic_scores
            .grid
            .row_major_locations()
            .map(move |location| (location, *scenic_scores.grid.get(&location).unwrap()))
    }
}

impl TryFrom<&str> for TreePatch {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parse_height = |c: char| -> anyhow::Result<Height> {
            match c {
                '0'..='9' => Ok(c as u8 - b'0'),
                _ => Err(anyhow!("invalid digit: {:?}", c)),
            }
        };

        let parse_line = |s: &str| -> anyhow::Result<Vec<Height>> {
            s.chars()
                .enumerate()
                .map(|(i, c)| parse_height(c).with_context(|| format!("column number {}", i + 1)))
                .collect::<Result<Vec<u8>, _>>()
        };

        let values = s
            .lines()
            .enumerate()
            .map(|(i, s)| parse_line(s).with_context(|| format!("row number {}", i + 1)))
            .collect::<Result<Vec<Vec<_>>, _>>()?;

        let grid = Grid::try_from(values).context("unable to parse height grid")?;

        Ok(Self { grid })
    }
}

impl std::fmt::Display for TreePatch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.grid.row_groups() {
            for location in row {
                write!(f, "{}", self.grid.get(&location).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn identity() -> anyhow::Result<()> {
        assert_eq!(INPUT, format!("{}", TreePatch::try_from(INPUT)?));
        Ok(())
    }
}
