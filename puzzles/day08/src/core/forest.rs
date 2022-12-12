use std::convert::TryFrom;

use anyhow::{anyhow, Context};

use super::{
    Cell, Grid, ScenicScore, ScenicScoreGrid, ScenicScores, Visibility, VisibilityGrid, DIRECTIONS,
};

pub type Height = u8;
pub type TreeCell<'a> = Cell<'a, Height>;
pub type TreePatch = Grid<Height>;

impl TryFrom<&str> for TreePatch {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parse_height = |c: char| -> anyhow::Result<Height> {
            match c {
                '0'..='9' => Ok(c as u8 - '0' as u8),
                _ => Err(anyhow!("invalid digit: {:?}", c)),
            }
        };

        let parse_line = |s: &str| -> anyhow::Result<Vec<Height>> {
            s.chars()
                .enumerate()
                .map(|(i, c)| parse_height(c).with_context(|| format!("column number {}", i + 1)))
                .collect::<Result<Vec<u8>, _>>()
        };

        Grid::<u8>::try_from(
            s.lines()
                .enumerate()
                .map(|(i, s)| parse_line(s).with_context(|| format!("row number {}", i + 1)))
                .collect::<Result<Vec<Vec<_>>, _>>()
                .context("unable to parse height grid")?,
        )
    }
}

impl std::fmt::Display for TreePatch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.rows() {
            for (_, value) in row {
                write!(f, "{}", value)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Visibility for TreePatch {
    fn visibility(&self) -> VisibilityGrid {
        fn scan_line<'a, L>(visibility: &mut VisibilityGrid, mut line: L)
        where
            L: Iterator<Item = &'a TreeCell<'a>>,
        {
            let (border, &height) = line.next().unwrap();
            *visibility.get_mut(&border).unwrap() = true;

            let mut max_height = height;

            for (location, &height) in line {
                if height > max_height {
                    *visibility.get_mut(&location).unwrap() = true;
                    max_height = height;
                }
            }
        }

        fn scan_axis<'a, A, L>(visibility: &mut VisibilityGrid, axis: A)
        where
            A: Iterator<Item = L>,
            L: Iterator<Item = TreeCell<'a>>,
        {
            for line in axis {
                let cells: Vec<_> = line.collect();
                scan_line(visibility, cells.iter());
                scan_line(visibility, cells.iter().rev());
            }
        }

        let mut visibility = VisibilityGrid::sized(self.size);

        scan_axis(&mut visibility, self.rows());
        scan_axis(&mut visibility, self.columns());

        visibility
    }
}

impl ScenicScores for TreePatch {
    fn scenic_scores(&self) -> ScenicScoreGrid {
        fn viewing_distance<'a>(mut vector: impl Iterator<Item = TreeCell<'a>>) -> ScenicScore {
            let maybe_start = vector.next();

            if maybe_start.is_none() {
                return 0;
            }

            let (_, &start_height) = maybe_start.unwrap();

            let mut count = 0;

            for (_, &height) in vector {
                count += 1;
                if height >= start_height {
                    break;
                }
            }

            count
        }

        let mut scenic_scores = ScenicScoreGrid::filled(self.size, 1);

        for (location, _) in self.cells() {
            for direction in DIRECTIONS {
                let vector = self.vector(&location, &direction);
                let score = viewing_distance(vector);
                *scenic_scores.get_mut(&location).unwrap() *= score;
            }
        }

        scenic_scores
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
