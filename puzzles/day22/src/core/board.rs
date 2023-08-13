use std::collections::HashMap;

use super::{CardinalDirection, Location, Map, Tile, Walk, Walker};

use CardinalDirection::*;

type Portals = HashMap<Location, Location>;

#[derive(Debug, Clone)]
pub struct Board {
    map: Map,
    vertical_portals: Portals,
    horizontal_portals: Portals,
}

impl<'a> Walk<'a> for Board {
    fn walker(&self) -> Walker {
        self.map.walker()
    }

    fn portal(&self, loc: Location, dir: CardinalDirection) -> (Location, CardinalDirection, Tile) {
        let portals = match dir {
            North | South => &self.vertical_portals,
            West | East => &self.horizontal_portals,
        };
        let &loc = portals.get(&loc).unwrap();
        (loc, dir, self.map.grid[loc].unwrap())
    }
}

impl From<Map> for Board {
    fn from(map: Map) -> Self {
        let (height, width) = map.grid.shape();

        let mut horizontal_portals = HashMap::new();

        for row in 0..height {
            if let Some(first_col) = (0..width).find(|&column| map.grid[(row, column)].is_some()) {
                let last_col = (0..width)
                    .rev()
                    .find(|&column| map.grid[(row, column)].is_some())
                    .unwrap_or(first_col);

                let first = (row, first_col);
                let last = (row, last_col);

                horizontal_portals.insert(first, last);
                horizontal_portals.insert(last, first);
            }
        }

        let mut vertical_portals = HashMap::new();

        for column in 0..width {
            if let Some(first_row) = (0..height).find(|&row| map.grid[(row, column)].is_some()) {
                let last_row = (0..height)
                    .rev()
                    .find(|&row| map.grid[(row, column)].is_some())
                    .unwrap_or(first_row);

                let first = (first_row, column);
                let last = (last_row, column);

                vertical_portals.insert(first, last);
                vertical_portals.insert(last, first);
            }
        }

        Self {
            map,
            vertical_portals,
            horizontal_portals,
        }
    }
}
