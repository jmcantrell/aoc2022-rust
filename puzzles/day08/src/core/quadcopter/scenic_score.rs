use geometry::{AxesBounds, CardinalDirection, Grid};

pub type ScenicScore = usize;

#[derive(Debug, Clone)]
pub struct ScenicScores {
    pub grid: Grid<ScenicScore>,
}

impl ScenicScores {
    pub fn new(grid: Grid<ScenicScore>) -> Self {
        Self { grid }
    }
}

impl<T: PartialOrd> From<&Grid<T>> for ScenicScores {
    fn from(grid: &Grid<T>) -> Self {
        let mut scenic_scores = Self {
            grid: Grid::filled(grid.size(), 1),
        };

        use CardinalDirection::*;
        let directions = [North, East, South, West];

        for start in grid.row_major_locations() {
            let start_height = grid.get(&start).unwrap();

            for direction in directions {
                let mut score = 0;
                let mut location = start;

                while let Some(adjacent) = grid.neighbor(&location, &direction) {
                    score += 1;
                    location = adjacent;

                    if grid.get(&adjacent).unwrap() >= start_height {
                        break;
                    }
                }

                *scenic_scores.grid.get_mut(&start).unwrap() *= score;
            }
        }

        scenic_scores
    }
}

impl From<ScenicScores> for Grid<ScenicScore> {
    fn from(scenic_scores: ScenicScores) -> Grid<ScenicScore> {
        scenic_scores.grid
    }
}
