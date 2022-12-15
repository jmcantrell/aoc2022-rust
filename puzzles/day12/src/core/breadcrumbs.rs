use std::collections::{hash_map::Entry, HashMap, VecDeque};

use crate::core::{Cell, Grid, Location};

pub type Path = Vec<Location>;

#[derive(Debug, Default, Clone)]
pub struct BreadCrumbs {
    target: Location,
    came_from: HashMap<Location, Location>,
}

impl BreadCrumbs {
    pub fn from_grid<T, F>(grid: &Grid<T>, target: Location, is_traversable: F) -> Self
    where
        F: Fn(Cell<T>, Cell<T>) -> bool,
    {
        let mut frontier = VecDeque::new();
        let mut came_from = HashMap::new();

        frontier.push_back(target);

        while let Some(current) = frontier.pop_front() {
            let current_value = grid.get(&current).unwrap();
            for (next, next_value) in grid.neighbors(&current) {
                if is_traversable((current, current_value), (next, next_value)) {
                    if let Entry::Vacant(entry) = came_from.entry(next) {
                        entry.insert(current);
                        frontier.push_back(next);
                    }
                }
            }
        }

        Self { target, came_from }
    }

    pub fn path(&self, from: Location) -> Option<Path> {
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
