use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::core::cube_net::CubeNet;
use crate::core::geometry::{AxesBounds, Direction, Location, DIRECTIONS};

use super::{Map, Tile, Walk, Walker};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IndexTransform {
    Row,
    Column,
    First,
    Last,
    InverseRow,
    InverseColumn,
}

impl IndexTransform {
    pub fn eval(&self, location: &Location, size: usize) -> usize {
        match self {
            Row => location.row,
            Column => location.column,
            First => 0,
            Last => size - 1,
            InverseRow => size - location.row - 1,
            InverseColumn => size - location.column - 1,
        }
    }
}

use Direction::*;
use IndexTransform::*;

lazy_static! {
    static ref EDGE_TRANSITIONS: HashMap<(Direction, Direction), (IndexTransform, IndexTransform)> =
        [
            ((East, East), (InverseRow, Last)),
            ((East, North), (First, InverseRow)),
            ((East, South), (Last, Row)),
            ((East, West), (Row, First)),
            ((North, East), (InverseColumn, Last)),
            ((North, North), (First, InverseColumn)),
            ((North, South), (Last, Column)),
            ((North, West), (Column, First)),
            ((South, East), (Column, Last)),
            ((South, North), (First, Column)),
            ((South, South), (Last, InverseColumn)),
            ((South, West), (InverseColumn, First)),
            ((West, East), (Row, Last)),
            ((West, North), (First, Row)),
            ((West, South), (Last, InverseRow)),
            ((West, West), (InverseRow, First)),
        ]
        .into_iter()
        .collect();
}

type Portals = HashMap<(Location, Direction), (Location, Direction)>;

#[derive(Debug, Clone)]
pub struct Cube {
    map: Map,
    portals: Portals,
}

impl Cube {}

impl Walk<'_> for Cube {
    fn walker(&self) -> Walker {
        self.map.walker()
    }

    fn neighbor(&self, location: Location, direction: Direction) -> (Location, Direction, Tile) {
        match self.map.grid.neighbor_cell_some(&location, &direction) {
            Some((adjacent, tile)) => (adjacent, direction, *tile),
            None => {
                let &(adjacent, direction) = self.portals.get(&(location, direction)).unwrap();
                (
                    adjacent,
                    direction,
                    *self.map.grid.get_some(&adjacent).unwrap(),
                )
            }
        }
    }
}

impl TryFrom<Map> for Cube {
    type Error = anyhow::Error;

    fn try_from(map: Map) -> Result<Self, Self::Error> {
        let net = CubeNet::try_from(&map.grid)?;

        let mut portals = HashMap::new();

        for (from, adjacency) in net.edges.iter() {
            let face = net.faces.get_some(from).unwrap();
            for edge in DIRECTIONS {
                let &(other_edge, to) = adjacency.get(&edge).unwrap();
                let other_face = net.faces.get_some(&to).unwrap();

                let (row_transform, column_transform) =
                    EDGE_TRANSITIONS.get(&(edge, other_edge)).unwrap();

                for edge_location in face.edge_locations(&edge) {
                    let other_edge_location = Location::new(
                        row_transform.eval(&edge_location, net.size),
                        column_transform.eval(&edge_location, net.size),
                    );

                    let a = face[&edge_location];
                    let b = other_face[&other_edge_location];

                    portals.insert((a, edge), (b, other_edge.inverse()));
                }
            }
        }

        Ok(Self { map, portals })
    }
}
