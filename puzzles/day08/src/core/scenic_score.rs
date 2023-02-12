use grid::Grid;

use super::{Direction, DIRECTIONS};

pub type ScenicScore = usize;

#[derive(Debug, Clone)]
pub struct ScenicScores {
    pub grid: Grid<ScenicScore>,
}

impl<T: PartialOrd> From<&Grid<T>> for ScenicScores {
    fn from(grid: &Grid<T>) -> Self {
        let (rows, cols) = grid.size();
        let mut results: Grid<ScenicScore> = Grid::init(rows, cols, 1);

        let neighbor = |row, col, dir: Direction| {
            let (adj_row, adj_col) = dir.neighbor(row, col)?;
            (adj_col < cols && adj_row < rows).then_some((adj_row, adj_col))
        };

        for (start_row, start_col) in (0..rows).flat_map(|row| (0..cols).map(move |col| (row, col))) {
            let start_height = &grid[start_row][start_col];

            for dir in DIRECTIONS {
                let mut score = 0;
                let mut row = start_row;
                let mut col = start_col;

                while let Some((adj_row, adj_col)) = neighbor(row, col, dir) {
                    score += 1;
                    row = adj_row;
                    col = adj_col;

                    if &grid[adj_row][adj_col] >= start_height {
                        break;
                    }
                }

                results[start_row][start_col] *= score;
            }
        }

        Self { grid: results }
    }
}
