use grid::Grid;

#[derive(Debug, Clone)]
pub struct Visibility {
    pub grid: Grid<bool>,
}

impl<T: PartialOrd> From<&Grid<T>> for Visibility {
    fn from(grid: &Grid<T>) -> Self {
        fn scan_line<T: PartialOrd>(
            results: &mut Grid<bool>,
            grid: &Grid<T>,
            mut line: impl Iterator<Item = (usize, usize)>,
        ) {
            let (row, col) = line.next().unwrap();
            *results.get_mut(row, col).unwrap() = true;
            let mut max_height = grid.get(row, col).unwrap();

            for (row, col) in line {
                let height = grid.get(row, col).unwrap();
                if height > max_height {
                    *results.get_mut(row, col).unwrap() = true;
                    max_height = height;
                }
            }
        }

        let (rows, cols) = grid.size();
        let mut results: Grid<bool> = Grid::new(rows, cols);

        for row in 0..rows {
            let line = (0..cols).map(|col| (row, col));
            scan_line(&mut results, grid, line.clone());
            scan_line(&mut results, grid, line.rev());
        }

        for col in 0..cols {
            let line = (0..rows).map(|row| (row, col));
            scan_line(&mut results, grid, line.clone());
            scan_line(&mut results, grid, line.rev());
        }

        Self { grid: results }
    }
}

impl From<Visibility> for Grid<bool> {
    fn from(visibility: Visibility) -> Grid<bool> {
        visibility.grid
    }
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..self.grid.rows() {
            for col in 0..self.grid.cols() {
                write!(f, "{}", self.grid[row][col] as u8)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
