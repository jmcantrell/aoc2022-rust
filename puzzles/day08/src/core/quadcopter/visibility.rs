use geometry::{AxesBounds, Grid, VectorGroupIterator, VectorIterator};

#[derive(Debug, Clone)]
pub struct Visibility {
    pub grid: Grid<bool>,
}

impl<T: PartialOrd> From<&Grid<T>> for Visibility {
    fn from(grid: &Grid<T>) -> Self {
        fn scan_group<T: PartialOrd>(
            visibility: &mut Visibility,
            grid: &Grid<T>,
            mut group: impl VectorIterator<usize>,
        ) {
            let border = group.next().unwrap();
            *visibility.grid.get_mut(&border).unwrap() = true;

            let mut max_height = grid.get(&border).unwrap();

            for location in group {
                let height = grid.get(&location).unwrap();
                if height > max_height {
                    *visibility.grid.get_mut(&location).unwrap() = true;
                    max_height = height;
                }
            }
        }

        fn scan_groups<T: PartialOrd>(
            visibility: &mut Visibility,
            grid: &Grid<T>,
            groups: impl VectorGroupIterator<usize>,
        ) {
            for group in groups {
                scan_group(visibility, grid, group.clone());
                scan_group(visibility, grid, group.rev());
            }
        }

        let mut visibility = Self {
            grid: Grid::<bool>::sized(grid.size()),
        };

        scan_groups(&mut visibility, grid, grid.row_groups());
        scan_groups(&mut visibility, grid, grid.column_groups());

        visibility
    }
}

impl From<Visibility> for Grid<bool> {
    fn from(visibility: Visibility) -> Grid<bool> {
        visibility.grid
    }
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.grid.row_groups() {
            for location in row {
                write!(
                    f,
                    "{}",
                    if *self.grid.get(&location).unwrap() {
                        '1'
                    } else {
                        '0'
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
