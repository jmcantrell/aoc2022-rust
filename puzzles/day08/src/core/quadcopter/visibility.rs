use crate::core::{Grid, Location};

pub type VisibilityGrid = Grid<bool>;

pub trait Visibility {
    fn visibility(&self) -> VisibilityGrid;
}

impl VisibilityGrid {
    pub fn visible<'a>(&'a self) -> impl Iterator<Item = Location> + 'a {
        self.cells().filter_map(
            |(location, &visible)| {
                if visible {
                    Some(location)
                } else {
                    None
                }
            },
        )
    }
}

impl std::fmt::Display for VisibilityGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.rows() {
            for (_, value) in row {
                write!(f, "{}", if *value { '1' } else { '0' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
