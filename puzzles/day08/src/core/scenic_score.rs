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

        let neighbor = |loc: Location, dir: Direction| {
            dir.neighbor(loc)
                .and_then(|loc| grid.get(loc).is_some().then_some(loc))
        };

        for start in (0..height).flat_map(|row| (0..width).map(move |column| (row, column))) {
            let start_height = &grid[start];

            for dir in DIRECTIONS {
                let mut score = 0;
                let mut loc = start;

                while let Some(adj) = neighbor(loc, dir) {
                    score += 1;
                    loc = adj;

                    if &grid[loc] >= start_height {
                        break;
                    }
                }

                results[start] *= score;
            }
        }

        Self { grid: results }
    }
}
