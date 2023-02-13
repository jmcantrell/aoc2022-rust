use std::collections::HashMap;

use lazy_static::lazy_static;

use super::{CardinalDirection, CubeNet, Location, Map, Tile, Walk, Walker};

use CardinalDirection::*;
use IndexTransform::*;

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
    pub fn eval(&self, (row, col): Location, size: usize) -> usize {
        match self {
            Row => row,
            Column => col,
            First => 0,
            Last => size - 1,
            InverseRow => size - row - 1,
            InverseColumn => size - col - 1,
        }
    }
}

lazy_static! {
    static ref EDGE_TRANSITIONS: HashMap<(CardinalDirection, CardinalDirection), (IndexTransform, IndexTransform)> =
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

type Portals = HashMap<(Location, CardinalDirection), (Location, CardinalDirection)>;

#[derive(Debug, Clone)]
pub struct Cube {
    map: Map,
    portals: Portals,
}

impl Walk<'_> for Cube {
    fn walker(&self) -> Walker {
        self.map.walker()
    }

    fn portal(&self, loc: Location, dir: CardinalDirection) -> (Location, CardinalDirection, Tile) {
        let &(loc, dir) = self.portals.get(&(loc, dir)).unwrap();
        (loc, dir, self.map.grid[loc].unwrap())
    }
}

impl TryFrom<Map> for Cube {
    type Error = anyhow::Error;

    fn try_from(map: Map) -> Result<Self, Self::Error> {
        let net = CubeNet::try_from(&map.grid)?;

        let mut portals = HashMap::new();

        let directions = [North, South, West, East];

        for (&from, adjacency) in net.edges.iter() {
            let face = net.faces[from].as_ref().unwrap();

            let (nrows, ncols) = face.shape();

            for edge in directions {
                let &(other_edge, to) = adjacency.get(&edge).unwrap();
                let other_face = net.faces[to].as_ref().unwrap();

                let (row_transform, column_transform) =
                    EDGE_TRANSITIONS.get(&(edge, other_edge)).unwrap();

                let (rr, cr) = match edge {
                    North => (0..1, 0..ncols),
                    South => ((nrows - 1)..nrows, 0..ncols),
                    West => (0..nrows, 0..1),
                    East => (0..nrows, (ncols - 1)..ncols),
                };

                for edge_location in rr.flat_map(|row| cr.clone().map(move |col| (row, col))) {
                    let other_edge_location = (
                        row_transform.eval(edge_location, net.size),
                        column_transform.eval(edge_location, net.size),
                    );

                    let a = face[edge_location];
                    let b = other_face[other_edge_location];

                    portals.insert((a, edge), (b, other_edge.inverse()));
                }
            }
        }

        Ok(Self { map, portals })
    }
}
