use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::fmt;
use std::ops::Range;

use anyhow::{anyhow, ensure, Context};
use lazy_static::lazy_static;

use crate::core::geometry::{
    AxesBounds, Direction, Grid, Location, Rectangle, RelativeDirection, Size, DIRECTIONS,
};
use crate::core::math::gcd;

const FACES_LEN: usize = 6;
const NET_DIMS: Size = Size::square(5);

type EdgeQuery = (RelativeDirection, Vec<RelativeDirection>);

use RelativeDirection::*;

lazy_static! {
    // I'm not sure if this approach is a precise or provable way to identify each of the eleven
    // possible patterns for a valid polyhedral net for a cube. This method does identify the
    // eleven variations, but I'm not sure if it will identify false positives.
    //
    // What I'm calling an edge query is just an encoding for a test of whether another face exists
    // in the net after following some pattern of moves away from the starting face. They're
    // encoded with relative directions so that they can be used for any edge surrounding the
    // starting face.
    //
    // The first item in each tuple is the edge of the target face that will be adjoining the
    // origin face relative to the edge of the origin face. For example, if we're trying to find
    // the face that adjoins the origin face at the north edge, if the first item of the tuple is
    // `Right`, then the edge of the candidate face that it will adjoin to will be the east edge,
    //
    // because east is to the right of north. The second item in the tuple is the path that needs
    // to be traversed to get to the candidate face. Each step in the path represents a turn that
    // needs to be made before moving a single step in the forward direction. For example, a `Left`
    // step means to first turn left, then take a single step forward. The starting direction is
    // the direction of the edge that is being matched.

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
pub type Adjacency = HashMap<Direction, (Direction, Location)>;
pub type Edges = HashMap<Location, Adjacency>;

#[derive(Debug, Clone)]
pub struct CubeNet {
    pub size: usize,
    pub faces: Faces,
    pub edges: Edges,
}

impl CubeNet {
    pub fn iter(&self) -> impl Iterator<Item = (Location, &Face, &Adjacency)> {
        self.faces.locations_by_row().filter_map(|location| {
            let face = self.faces.get_some(&location)?;
            let adjacency = self.edges.get(&location).unwrap();
            Some((location, face, adjacency))
        })
    }
}

impl AxesBounds for CubeNet {
    fn vertical_bounds(&self) -> Range<usize> {
        self.faces.vertical_bounds()
    }

    fn horizontal_bounds(&self) -> Range<usize> {
        self.faces.horizontal_bounds()
    }
}

impl fmt::Display for CubeNet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.faces.row_groups() {
            for location in row {
                write!(
                    f,
                    "{}",
                    match self.faces.get(&location).unwrap() {
                        Some(_) => '#',
                        None => '.',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn map_adjacency<T>(net: &Net<T>) -> Edges {
    let mut frontier: Vec<Location> = Vec::new();
    let mut reached: HashSet<Location> = HashSet::new();
    let mut edges = Edges::new();

    if let Some(start) = net
        .locations_by_row()
        .find(|location| net.contains_some(location))
    {
        frontier.push(start);
        reached.insert(start);
    }

    while let Some(current) = frontier.pop() {
        for edge in DIRECTIONS {
            if let Some(next) = net.neighbor_some(&current, &edge) {
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
    for (&start, adjacency) in edges.iter_mut() {
        for edge in DIRECTIONS {
            if adjacency.contains_key(&edge) {
                continue;
            }

            'search: for (other_edge_relative, path) in EDGE_QUERIES.iter() {
                let mut location = start;
                let mut direction = edge;

                for turn_relative in path.iter() {
                    direction += *turn_relative;
                    if let Some(next_location) = net.neighbor_some(&location, &direction) {
                        location = next_location;
                    } else {
                        continue 'search;
                    }
                }

                let other_edge = edge + *other_edge_relative;

                if !net.has_some_neighbor(&location, &other_edge) {
                    match adjacency.entry(edge) {
                        Entry::Occupied(entry) => {
                            let (other_edge, end) = entry.get();
                            return Err(anyhow!("existing {edge} edge found for {start} => {other_edge} edge of {end}"));
                        }
                        Entry::Vacant(entry) => {
                            entry.insert((other_edge, location));
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
        let inner = net.bounds_some().context("net is empty")?;

        let size = gcd(inner.height(), inner.width());
        let face_size = Size::square(size);
        let face_area = face_size.area();

        let net_size = NET_DIMS * face_size;
        let mut faces: Vec<Option<Face>> = Vec::with_capacity(net_size.area());

        let inner_offset = Size::from(inner.top_left);

        for top_left in NET_DIMS
            .locations_by_row()
            .map(|start| start * face_size + inner_offset)
        {
            if !net.contains_some(&top_left) {
                faces.push(None);
                continue;
            }

            let face_bounds: Rectangle = (top_left, face_size).into();
            let mut face_values: Vec<Location> = Vec::with_capacity(face_area);

            for location in face_bounds.locations_by_row() {
                ensure!(
                    net.contains_some(&location),
                    "expected a non-empty cell at {}",
                    location
                );

                face_values.push(location);
            }

            faces.push(Some(Face::try_from((face_size, face_values))?));
        }

        let faces_len = faces.iter().filter(|value| value.is_some()).count();

        ensure!(
            faces_len == FACES_LEN,
            "expected net pattern to have {FACES_LEN} faces, but it had {faces_len}",
        );

        let faces = Net::try_from((NET_DIMS, faces))?;

        let edges = map_adjacency(&faces);

        ensure!(
            edges.keys().count() == FACES_LEN,
            "net pattern has disconnected faces"
        );

        let edges = try_wrap_edges(&faces, edges)?;

        for (face, adjacency) in edges.iter() {
            for edge in DIRECTIONS {
                ensure!(
                    adjacency.contains_key(&edge),
                    "face at {face} is missing its {edge} edge"
                );
            }
        }

        Ok(Self { size, faces, edges })
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::core::geometry::index_for_location;

    use super::*;

    type NetKey = u32;

    lazy_static! {
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
        let net_len = NET_DIMS.area();
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

        for value in net.values() {
            key <<= 1;
            if value.is_some() {
                key |= 1;
            }
        }

        key
    }

    fn net_from_key(id: usize, key: NetKey) -> Net<usize> {
        let values = split_net_key(key)
            .into_iter()
            .map(|is| if is { Some(id) } else { None })
            .collect();

        trim_net(Net::try_from((NET_DIMS, values)).unwrap())
    }

    fn trim_net<T>(mut net: Net<T>) -> Net<T> {
        let inner = net.bounds_some().unwrap();

        let values = inner
            .locations_by_row()
            .map(|location| net.get_mut(&location).unwrap().take())
            .collect();

        Net::try_from((inner.size(), values)).unwrap()
    }

    fn transpose_net<T: Clone>(net: Net<T>) -> Net<T> {
        let size = net.size().transpose();
        let mut values = vec![None; size.area()];

        for (location, value) in net {
            values[dbg!(index_for_location(&location.transpose(), size.width))] = value;
        }

        Net::try_from((size, values)).unwrap()
    }

    fn flip_net<T>(mut net: Net<T>) -> Net<T> {
        for row in net.row_groups().take(net.height() / 2) {
            for location in row {
                net.swap(
                    &location,
                    &Location::new(net.bottom() - location.row, location.column),
                );
            }
        }
        net
    }

    fn valid_cube_nets() -> impl Iterator<Item = (usize, Net<usize>)> {
        VALID_CUBE_NETS
            .into_iter()
            .enumerate()
            .flat_map(|(shape, key)| {
                let mut net = net_from_key(shape, key);

                let mut variations = Vec::with_capacity(DIRECTIONS.len() * 2);

                for _ in DIRECTIONS {
                    variations.push((shape, trim_net(net.clone())));
                    net = transpose_net(net);
                    variations.push((shape, trim_net(net.clone())));
                    net = flip_net(net);
                }

                variations.into_iter()
            })
    }

    fn possible_nets() -> impl Iterator<Item = Net<usize>> {
        let n = NET_DIMS.area();
        (0..n).combinations(FACES_LEN).flat_map(move |positions| {
            let mut values = vec![None; n];

            for (j, i) in positions.into_iter().enumerate() {
                values[i] = Some(j);
            }

            let mut net = Net::try_from((NET_DIMS, values)).unwrap();
            let mut variations = Vec::with_capacity(DIRECTIONS.len() * 2);

            for _ in DIRECTIONS {
                variations.push(trim_net(net.clone()));
                net = transpose_net(net);
                variations.push(trim_net(net.clone()));
                net = flip_net(net);
            }

            variations.into_iter()
        })
    }

    fn invalid_cube_nets() -> impl Iterator<Item = Net<usize>> {
        let valid_cube_nets: HashSet<NetKey> = valid_cube_nets()
            .map(|(_, net)| key_from_net(&trim_net(net)))
            .collect();

        possible_nets().filter_map(move |net| {
            let net = trim_net(net);
            let key = key_from_net(&net);
            if !valid_cube_nets.contains(&key) {
                Some(net)
            } else {
                None
            }
        })
    }

    fn describe_net<T>(net: &Net<T>) -> String {
        let mut description = String::new();

        for row in net.row_groups() {
            for location in row {
                description.push(match net.get(&location).unwrap() {
                    Some(_) => '#',
                    None => '.',
                });
            }
            description.push('\n');
        }

        description
    }

    #[test]
    #[ignore]
    fn valid_cube_nets_are_recognized() -> anyhow::Result<()> {
        for (shape, net) in valid_cube_nets() {
            CubeNet::try_from(&net).with_context(|| {
                format!("cube net shape {}:\n{}", shape + 1, describe_net(&net))
            })?;
        }
        Ok(())
    }

    #[test]
    #[ignore]
    fn invalid_cube_nets_are_not_recognized() -> anyhow::Result<()> {
        for net in invalid_cube_nets() {
            if let Ok(cube_net) = CubeNet::try_from(&net) {
                dbg!(cube_net);
                return Err(anyhow!(
                    "net shape falsely recognized as valid:\n{}",
                    describe_net(&net)
                ));
            }
        }
        Ok(())
    }
}
