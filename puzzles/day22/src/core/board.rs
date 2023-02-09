use std::collections::HashMap;

use crate::geometry::{Direction, Location};

use super::{Map, Tile, Walk, Walker};

type Portals = HashMap<Location, Location>;

#[derive(Debug, Clone)]
pub struct Board {
    map: Map,
    vertical_portals: Portals,
    horizontal_portals: Portals,
}

impl Board {
    fn portal(&self, location: &Location, direction: &Direction) -> Location {
        *match direction {
            Direction::East | Direction::West => self.horizontal_portals.get(location),
            Direction::North | Direction::South => self.vertical_portals.get(location),
        }
        .unwrap()
    }
}

impl<'a> Walk<'a> for Board {
    fn walker(&self) -> Walker {
        self.map.walker()
    }

    fn neighbor(&self, location: Location, direction: Direction) -> (Location, Direction, Tile) {
        match self.map.grid.neighbor_cell_some(&location, &direction) {
            Some((adjacent, tile)) => (adjacent, direction, *tile),
            None => {
                let adjacent = self.portal(&location, &direction);
                (
                    adjacent,
                    direction,
                    *self.map.grid.get_some(&adjacent).unwrap(),
                )
            }
        }
    }
}

impl From<Map> for Board {
    fn from(map: Map) -> Self {
        fn insert_extents(mut map: Portals, (first, last): (Location, Location)) -> Portals {
            map.insert(first, last);
            map.insert(last, first);
            map
        }

        let vertical_portals = map
            .grid
            .vertical_bounds_by_column()
            .fold(HashMap::new(), insert_extents);

        let horizontal_portals = map
            .grid
            .horizontal_bounds_by_row()
            .fold(HashMap::new(), insert_extents);

        Self {
            map,
            vertical_portals,
            horizontal_portals,
        }
    }
}
