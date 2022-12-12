use crate::core::Grid;

pub type ScenicScore = usize;
pub type ScenicScoreGrid = Grid<ScenicScore>;

pub trait ScenicScores {
    fn scenic_scores(&self) -> ScenicScoreGrid;
}
