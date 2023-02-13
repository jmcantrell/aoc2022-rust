use anyhow::{anyhow, ensure, Context};

use super::{Grid, ScenicScore, ScenicScores, Visibility};

pub type Height = u8;

#[derive(Debug, Clone)]
pub struct TreePatch {
    pub grid: Grid<Height>,
}

impl TreePatch {
    fn iter_locations(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.grid.nrows())
            .flat_map(move |row| (0..self.grid.ncols()).map(move |col| (row, col)))
    }
}

impl TreePatch {
    pub fn count_visible(&self) -> usize {
        let visibility = Visibility::from(&self.grid);
        self.iter_locations()
            .filter(|loc| visibility.grid[*loc])
            .count()
    }

    pub fn max_scenic_score(&self) -> ScenicScore {
        let scenic_scores = ScenicScores::from(&self.grid);
        self.iter_locations()
            .map(|loc| scenic_scores.grid[loc])
            .max()
            .unwrap_or_default()
    }
}

impl TryFrom<&str> for TreePatch {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parse_height = |c: char| -> anyhow::Result<Height> {
            match c {
                '0'..='9' => Ok(c as Height - b'0'),
                _ => Err(anyhow!("invalid digit: {:?}", c)),
            }
        };

        let parse_line = |s: &str| -> anyhow::Result<Vec<Height>> {
            s.chars()
                .enumerate()
                .map(|(i, c)| parse_height(c).with_context(|| format!("column number {}", i + 1)))
                .collect::<Result<Vec<Height>, _>>()
        };

        let rows = s
            .lines()
            .enumerate()
            .map(|(i, s)| parse_line(s).with_context(|| format!("row number {}", i + 1)))
            .collect::<Result<Vec<Vec<_>>, _>>()?;

        ensure!(!rows.is_empty(), "grid is empty");

        let nrows = rows.len();
        let ncols = rows[0].len();

        for (i, row) in rows.iter().skip(1).enumerate() {
            ensure!(
                row.len() == ncols,
                "expected row number {} to have {} columns, but it had {}",
                i + 1,
                ncols,
                row.len()
            );
        }

        let grid = Grid::from_row_iterator(
            nrows,
            ncols,
            rows.into_iter().flat_map(|row| row.into_iter()),
        );

        Ok(Self { grid })
    }
}

impl std::fmt::Display for TreePatch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.grid.row_iter() {
            for value in row.iter() {
                write!(f, "{value}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
