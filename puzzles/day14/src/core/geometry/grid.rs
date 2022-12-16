use std::collections::HashMap;

use super::Location;

pub type Cell<'a, T> = (&'a Location, &'a T);

#[derive(Debug, Clone)]
pub struct GraphGrid<T>(HashMap<Location, T>);

impl<T> GraphGrid<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, location: &Location) -> Option<&T> {
        self.0.get(location)
    }

    pub fn insert(&mut self, location: Location, value: T) -> Option<T> {
        self.0.insert(location, value)
    }

    pub fn contains(&self, location: &Location) -> bool {
        self.0.contains_key(location)
    }

    pub fn locations(&self) -> impl Iterator<Item = &Location> {
        self.0.keys()
    }

    pub fn extents(&self) -> (Location, Location) {
        if self.0.is_empty() {
            (Location::default(), Location::default())
        } else {
            let mut locations = self.0.keys();

            let first = locations.next().unwrap();
            let mut top_left = *first;
            let mut bottom_right = *first;

            for &location in locations {
                if location.row < top_left.row {
                    top_left.row = location.row;
                }
                if location.column < top_left.column {
                    top_left.column = location.column;
                }
                if location.row > bottom_right.row {
                    bottom_right.row = location.row;
                }
                if location.column > bottom_right.column {
                    bottom_right.column = location.column;
                }
            }

            (top_left, bottom_right)
        }
    }

    pub fn bottom(&self) -> isize {
        self.extents().1.row
    }
}
