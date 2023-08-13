use super::Grid;

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
            let loc = line.next().unwrap();
            results[loc] = true;

            let mut max_height = &grid[loc];

            for loc in line {
                let height = &grid[loc];
                if height > max_height {
                    results[loc] = true;
                    max_height = height;
                }
            }
        }

        let (height, width) = grid.shape();
        let mut results: Grid<bool> = Grid::from_element(height, width, false);

        for row in 0..height {
            let line = (0..width).map(|column| (row, column));
            scan_line(&mut results, grid, line.clone());
            scan_line(&mut results, grid, line.rev());
        }

        for column in 0..width {
            let line = (0..height).map(|row| (row, column));
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
        for row in self.grid.row_iter() {
            for value in row.iter() {
                write!(f, "{}", *value as u8)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
