use super::{Direction, Grid, Location, DIRECTIONS};

pub type ScenicScore = usize;

#[derive(Debug, Clone)]
pub struct ScenicScores {
    pub grid: Grid<ScenicScore>,
}

impl<T: PartialOrd> From<&Grid<T>> for ScenicScores {
    fn from(grid: &Grid<T>) -> Self {
        let (height, width) = grid.shape();
        let mut results: Grid<ScenicScore> = Grid::from_element(height, width, 1);

        let neighbor = |location: Location, direction: Direction| {
            direction
                .neighbor(location)
                .and_then(|location| grid.get(location).is_some().then_some(location))
        };

        for start in (0..height).flat_map(|row| (0..width).map(move |column| (row, column))) {
            let start_height = &grid[start];

            for direction in DIRECTIONS {
                let mut score = 0;
                let mut location = start;

                while let Some(adj) = neighbor(location, direction) {
                    score += 1;
                    location = adj;

                    if &grid[location] >= start_height {
                        break;
                    }
                }

                results[start] *= score;
            }
        }

        Self { grid: results }
    }
}
