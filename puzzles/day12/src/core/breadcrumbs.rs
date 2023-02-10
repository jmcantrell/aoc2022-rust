use std::collections::{hash_map::Entry, HashMap, VecDeque};

use geometry::{AxesBounds, CardinalDirection, Grid, GridLocation};

pub type Path = Vec<GridLocation>;

use CardinalDirection::*;
const DIRECTIONS: [CardinalDirection; 4] = [North, South, West, East];

#[derive(Debug, Default, Clone)]
pub struct BreadCrumbs {
    target: GridLocation,
    came_from: HashMap<GridLocation, GridLocation>,
}

impl BreadCrumbs {
    pub fn from_grid<T, F>(grid: &Grid<T>, target: GridLocation, is_traversable: F) -> Self
    where
        F: Fn(&T, &T) -> bool,
    {
        let mut frontier = VecDeque::new();
        let mut came_from = HashMap::new();

        frontier.push_back(target);

        while let Some(current) = frontier.pop_front() {
            let current_value = grid.get(&current).unwrap();
            for next in DIRECTIONS
                .iter()
                .filter_map(|direction| grid.neighbor(&current, direction))
            {
                let next_value = grid.get(&next).unwrap();

                if is_traversable(current_value, next_value) {
                    if let Entry::Vacant(entry) = came_from.entry(next) {
                        entry.insert(current);
                        frontier.push_back(next);
                    }
                }
            }
        }

        Self { target, came_from }
    }

    pub fn path(&self, from: GridLocation) -> Option<Path> {
        let mut current = from;
        let mut path: Vec<_> = Default::default();

        while current != self.target {
            path.push(current);
            current = *self.came_from.get(&current)?;
        }

        path.reverse();

        Some(path)
    }
}
