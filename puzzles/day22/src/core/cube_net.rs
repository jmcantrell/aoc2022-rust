use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::fmt;

use anyhow::{anyhow, ensure, Context};
use lazy_static::lazy_static;
use num::integer::gcd;

use super::{CardinalDirection, Grid, Location, RelativeDirection, Size};

use CardinalDirection::*;
use RelativeDirection::*;

const FACES_LEN: usize = 6;
const NET_SIZE: usize = 5;
const NET_DIMS: Size = Size::new(NET_SIZE, NET_SIZE);
const DIRECTIONS: [CardinalDirection; 4] = [North, South, West, East];

type EdgeQuery = (RelativeDirection, Vec<RelativeDirection>);

lazy_static! {
    /// I'm not sure if this approach is a precise or provable way to validate a polyhedral net as
    /// one of the possible patterns for a cube. This method does identify the eleven variations,
    /// but I'm not sure if it will identify false positives.
    ///
    /// What I'm calling an edge query is just an encoding for a test of whether another face exists
    /// in the net after following some pattern of moves away from the starting face. They're
    /// encoded with relative directions so that they can be used for any edge surrounding the
    /// starting face.
    ///
    /// The first item in each tuple is the edge of the target face that will be adjoining the
    /// origin face relative to the edge of the origin face. For example, if we're trying to find
    /// the face that adjoins the origin face at the north edge, if the first item of the tuple is
    /// `Right`, then the edge of the candidate face that it will adjoin to will be the east edge,
    /// because east is to the right of north.
    ///
    /// The second item in the tuple is the path that needs to be traversed to get to the candidate
    /// face. Each step in the path represents a turn that needs to be made before moving a single
    /// step forward. For example, a `Left` step means to first turn left, then take a single step
    /// forward. The starting direction is the direction of the edge that is being matched.

    static ref EDGE_QUERIES: Vec<EdgeQuery> = vec![
        (Backward, vec![Backward, Forward, Forward]),
        (Backward, vec![Backward, Left, Right, Forward, Left]),
        (Backward, vec![Backward, Right, Left, Forward, Right]),
        (Backward, vec![Left, Forward, Left, Right, Forward]),
        (Backward, vec![Left, Left, Forward, Right, Left]),
        (Backward, vec![Left, Left, Right, Left, Right]),
        (Backward, vec![Right, Forward, Right, Left, Forward]),
        (Backward, vec![Right, Right, Forward, Left, Right]),
        (Backward, vec![Right, Right, Left, Right, Left]),
        (Forward, vec![Backward, Left, Forward]),
        (Forward, vec![Backward, Right, Forward]),
        (Forward, vec![Left, Forward, Forward, Right]),
        (Forward, vec![Left, Forward, Right]),
        (Forward, vec![Right, Forward, Forward, Left]),
        (Forward, vec![Right, Forward, Left]),
        (Left, vec![Backward, Forward, Right, Left]),
        (Left, vec![Backward, Right, Left, Right]),
        (Left, vec![Left, Left, Right, Forward]),
        (Left, vec![Right, Left]),
        (Left, vec![Right, Right, Forward, Forward]),
        (Right, vec![Backward, Forward, Left, Right]),
        (Right, vec![Backward, Left, Right, Left]),
        (Right, vec![Left, Left, Forward, Forward]),
        (Right, vec![Left, Right]),
        (Right, vec![Right, Right, Left, Forward]),
    ];
}

pub type Net<T> = Grid<Option<T>>;
pub type Face = Grid<Location>;
pub type Faces = Net<Face>;
pub type Adjacency = HashMap<CardinalDirection, (CardinalDirection, Location)>;
pub type Edges = HashMap<Location, Adjacency>;

#[derive(Debug, Clone)]
pub struct CubeNet {
    pub size: usize,
    pub faces: Faces,
    pub edges: Edges,
}

fn describe_net<T>(net: &Net<T>) -> String {
    let mut description = String::new();

    for row in net.row_iter() {
        for maybe_value in row.iter() {
            description.push(match maybe_value {
                Some(_) => '#',
                None => '.',
            });
        }
        description.push('\n');
    }

    description
}

impl fmt::Display for CubeNet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", describe_net(&self.faces))
    }
}

fn inner_some<T>(grid: &Grid<Option<T>>) -> Option<(Location, Location)> {
    let (height, width) = grid.shape();

    let top = (0..height).find(|&i| grid.row(i).iter().any(|maybe_value| maybe_value.is_some()))?;

    let bottom = (0..height)
        .rev()
        .find(|&i| grid.row(i).iter().any(|maybe_value| maybe_value.is_some()))
        .unwrap_or(top);

    let left = (0..width)
        .find(|&i| {
            grid.column(i)
                .iter()
                .any(|maybe_value| maybe_value.is_some())
        })
        .unwrap();

    let right = (0..width)
        .rev()
        .find(|&i| {
            grid.column(i)
                .iter()
                .any(|maybe_value| maybe_value.is_some())
        })
        .unwrap_or(left);

    Some(((top, left), (bottom, right)))
}

fn map_adjacency<T: std::fmt::Debug>(net: &Net<T>) -> Edges {
    let mut frontier: Vec<Location> = Vec::new();
    let mut reached: HashSet<Location> = HashSet::new();
    let mut edges = Edges::new();

    let neighbor = |loc: Location, dir: CardinalDirection| -> Option<Location> {
        dir.neighbor(loc).and_then(|adj| {
            net.get(adj)
                .and_then(|maybe_value| maybe_value.is_some().then_some(adj))
        })
    };

    if let Some(start) = (0..net.nrows())
        .flat_map(|row| (0..net.ncols()).map(move |column| (row, column)))
        .find(|&loc| net[loc].is_some())
    {
        frontier.push(start);
        reached.insert(start);
    }

    while let Some(current) = frontier.pop() {
        for edge in DIRECTIONS {
            if let Some(next) = neighbor(current, edge) {
                edges
                    .entry(current)
                    .or_default()
                    .insert(edge, (edge.inverse(), next));
                if !reached.contains(&next) {
                    frontier.push(next);
                    reached.insert(next);
                }
            }
        }
    }

    edges
}

fn try_wrap_edges<T>(net: &Net<T>, mut edges: Edges) -> anyhow::Result<Edges> {
    let neighbor = |loc: Location, dir: CardinalDirection| -> Option<Location> {
        dir.neighbor(loc).and_then(|adj| {
            net.get(adj)
                .and_then(|maybe_value| maybe_value.is_some().then_some(adj))
        })
    };

    for (&start, adjacency) in edges.iter_mut() {
        for edge in DIRECTIONS {
            if adjacency.contains_key(&edge) {
                continue;
            }

            'search: for (other_edge_relative, path) in EDGE_QUERIES.iter() {
                let mut loc = start;
                let mut dir = edge;

                for turn in path.iter() {
                    dir += *turn;
                    if let Some(next_loc) = neighbor(loc, dir) {
                        loc = next_loc;
                    } else {
                        continue 'search;
                    }
                }

                let other_edge = edge + *other_edge_relative;

                if neighbor(loc, other_edge).is_none() {
                    match adjacency.entry(edge) {
                        Entry::Occupied(entry) => {
                            let (other_edge, end) = entry.get();
                            return Err(anyhow!("existing {edge} edge found for row {} and column {} => {other_edge} edge of row {} and column {}", start.0, start.1, end.0, end.1));
                        }
                        Entry::Vacant(entry) => {
                            entry.insert((other_edge, loc));
                        }
                    }
                }
            }
        }
    }

    Ok(edges)
}

impl<T> TryFrom<&Net<T>> for CubeNet {
    type Error = anyhow::Error;

    fn try_from(net: &Net<T>) -> Result<Self, Self::Error> {
        let ((top, left), (bottom, right)) = inner_some(net).context("net is empty")?;

        let height = bottom - top + 1;
        let width = right - left + 1;

        let size = gcd(height, width);

        let face_area = size * size;

        let mut faces: Vec<Option<Face>> = Vec::with_capacity(NET_DIMS.product());

        for (face_top, face_left) in (top..(top + size * NET_SIZE))
            .step_by(size)
            .flat_map(|row| {
                (left..(left + size * NET_SIZE))
                    .step_by(size)
                    .map(move |column| (row, column))
            })
        {
            let face_top_left = (face_top, face_left);

            if net
                .get(face_top_left)
                .map(|maybe_value| maybe_value.as_ref())
                .unwrap_or_default()
                .is_none()
            {
                faces.push(None);
                continue;
            }

            let mut face = Vec::with_capacity(face_area);

            for (face_row, face_col) in (face_top..(face_top + size))
                .flat_map(|row| (face_left..(face_left + size)).map(move |column| (row, column)))
            {
                ensure!(
                    net[(face_row, face_col)].is_some(),
                    "expected a non-empty value at row {} and column {}",
                    face_row + 1,
                    face_col + 1
                );

                face.push((face_row, face_col));
            }

            faces.push(Some(Face::from_row_iterator(size, size, face.into_iter())));
        }

        let faces_len = faces
            .iter()
            .filter(|maybe_value| maybe_value.is_some())
            .count();

        ensure!(
            faces_len == FACES_LEN,
            "expected net pattern to have {FACES_LEN} faces, but it had {faces_len}",
        );

        let faces = Net::from_row_iterator(NET_SIZE, NET_SIZE, faces);

        let edges = map_adjacency(&faces);

        ensure!(
            edges.keys().count() == FACES_LEN,
            "net pattern has disconnected faces"
        );

        let edges = try_wrap_edges(&faces, edges)?;

        for ((row, column), adjacency) in edges.iter() {
            for edge in DIRECTIONS {
                ensure!(
                    adjacency.contains_key(&edge),
                    "face at row {} and column {} is missing its {edge} edge",
                    row + 1,
                    column + 1
                );
            }
        }

        Ok(Self { size, faces, edges })
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    type NetKey = u32;

    lazy_static! {
        /// The minimum space required for every possible polyhedral net that could possibly be a
        /// cube is a 5x5 grid. One way to represent this is a bit string of length 25. The
        /// following are the canonical representations of the eleven valid cube nets.

        static ref VALID_CUBE_NETS: [NetKey; 11] = [
            0b1110001000010000100000000,
            0b0110011000010000100000000,
            0b0110001000110000100000000,
            0b0110001000010001100000000,
            0b0100001100110000100000000,
            0b0100011100010000100000000,
            0b0010011100001100000000000,
            0b0010001100110000100000000,
            0b0110001000110001000000000,
            0b0010001100110001000000000,
            0b0100001000110001000010000,
        ];
    }

    fn split_net_key(mut key: NetKey) -> Vec<bool> {
        let net_len = NET_DIMS.x * NET_DIMS.y;
        let mut values = Vec::with_capacity(net_len);

        for _ in 0..net_len {
            values.push(key & 1 != 0);
            key >>= 1;
        }

        values.reverse();
        values
    }

    fn key_from_net<T>(net: &Net<T>) -> NetKey {
        let mut key = 0;

        for maybe_value in net.iter() {
            key <<= 1;
            if maybe_value.is_some() {
                key |= 1;
            }
        }

        key
    }

    fn net_from_key(id: usize, key: NetKey) -> Net<usize> {
        Net::from_row_iterator(
            NET_DIMS.y,
            NET_DIMS.x,
            split_net_key(key).into_iter().map(|is| is.then_some(id)),
        )
    }

    fn flip_net(net: &mut Net<usize>) {
        let n = net.nrows();
        for i in 0..(n / 2) {
            net.swap_rows(i, n - i - 1);
        }
    }

    fn trim_net(net: Net<usize>) -> Net<usize> {
        let ((top, left), (bottom, right)) = inner_some(&net).unwrap();
        Net::from(net.view((top, left), (bottom - top + 1, right - left + 1)))
    }

    /// This generates every possible valid cube net by taking the canonical representations
    /// defined earlier and extending that list with every flipped and rotate variation of each.

    fn valid_cube_nets() -> impl Iterator<Item = (usize, Net<usize>)> {
        VALID_CUBE_NETS
            .into_iter()
            .enumerate()
            .flat_map(|(shape, key)| {
                let mut net = net_from_key(shape, key);

                let mut variations = Vec::with_capacity(DIRECTIONS.len() * 2);

                for _ in DIRECTIONS {
                    variations.push((shape, trim_net(net.clone())));
                    net = net.transpose();
                    variations.push((shape, trim_net(net.clone())));
                    flip_net(&mut net);
                }

                variations.into_iter()
            })
    }

    /// This generates every possible polyhedral net that will fit in a 5x5 grid. Obviously this
    /// will include the valid cube nets as well, so they will have to be filtered out later.

    fn possible_nets() -> impl Iterator<Item = Net<usize>> {
        let n = NET_DIMS.x * NET_DIMS.y;
        (0..n).combinations(FACES_LEN).map(move |positions| {
            let mut values = vec![None; n];

            for (j, i) in positions.into_iter().enumerate() {
                values[i] = Some(j);
            }

            trim_net(Net::from_row_iterator(
                NET_DIMS.y,
                NET_DIMS.x,
                values.into_iter(),
            ))
        })
    }

    fn invalid_cube_nets() -> impl Iterator<Item = Net<usize>> {
        let valid_cube_nets: HashSet<NetKey> = valid_cube_nets()
            .map(|(_, net)| key_from_net(&net))
            .collect();

        possible_nets().filter_map(move |net| {
            let key = key_from_net(&net);
            (!valid_cube_nets.contains(&key)).then_some(net)
        })
    }

    #[test]
    fn valid_cube_nets_are_recognized() -> anyhow::Result<()> {
        for (shape, net) in valid_cube_nets() {
            CubeNet::try_from(&net).with_context(|| {
                format!("cube net shape {}:\n{}", shape + 1, describe_net(&net))
            })?;
        }
        Ok(())
    }

    #[test]
    fn invalid_cube_nets_are_not_recognized() -> anyhow::Result<()> {
        for net in invalid_cube_nets() {
            if CubeNet::try_from(&net).is_ok() {
                return Err(anyhow!(
                    "net shape falsely recognized as valid:\n{}",
                    describe_net(&net)
                ));
            }
        }
        Ok(())
    }
}
