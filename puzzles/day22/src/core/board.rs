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
        let (nrows, ncols) = map.grid.shape();

        let mut horizontal_portals = HashMap::new();

        for row in 0..nrows {
            if let Some(first_col) = (0..ncols).find(|&col| map.grid[(row, col)].is_some()) {
                let last_col = (0..ncols)
                    .rev()
                    .find(|&col| map.grid[(row, col)].is_some())
                    .unwrap_or(first_col);

                let first = (row, first_col);
                let last = (row, last_col);

                horizontal_portals.insert(first, last);
                horizontal_portals.insert(last, first);
            }
        }

        let mut vertical_portals = HashMap::new();

        for col in 0..ncols {
            if let Some(first_row) = (0..nrows).find(|&row| map.grid[(row, col)].is_some()) {
                let last_row = (0..nrows)
                    .rev()
                    .find(|&row| map.grid[(row, col)].is_some())
                    .unwrap_or(first_row);

                let first = (first_row, col);
                let last = (last_row, col);

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
