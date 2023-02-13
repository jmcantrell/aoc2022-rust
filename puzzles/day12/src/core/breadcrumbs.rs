use std::collections::{hash_map::Entry, HashMap, VecDeque};

use super::{Direction, Grid, Location, DIRECTIONS};

pub type Path = Vec<Location>;

#[derive(Debug, Default, Clone)]
pub struct BreadCrumbs {
    target: Location,
    came_from: HashMap<Location, Location>,
}

impl BreadCrumbs {
    pub fn from_grid<T, F>(grid: &Grid<T>, target: Location, is_traversable: F) -> Self
    where
        F: Fn(&T, &T) -> bool,
    {
        let mut frontier = VecDeque::new();
        frontier.push_back(target);

        let mut came_from = HashMap::new();

        let neighbor = |loc: &Location, dir: &Direction| {
            dir.neighbor(loc)
                .and_then(|loc| grid.get(loc).is_some().then_some(loc))
        };

        while let Some(current) = frontier.pop_front() {
            let current_value = &grid[current];

            for next in DIRECTIONS.iter().filter_map(|dir| neighbor(&current, dir)) {
                let next_value = &grid[next];

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
