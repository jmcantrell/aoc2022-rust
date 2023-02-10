use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::fmt;
use std::ops::Range;

use anyhow::{anyhow, ensure, Context};
use lazy_static::lazy_static;

use geometry::{AxesBounds, CardinalDirection, Grid, RelativeDirection};

use super::{Direction, Extents, Location, Size};

use CardinalDirection::*;
use RelativeDirection::*;

const FACES_LEN: usize = 6;
const NET_DIMS: Size = Size::square(5);
const DIRECTIONS: [Direction; 4] = [North, South, West, East];

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
        (Down, vec![Down, Left, Right, Up, Left]),
        (Down, vec![Down, Right, Left, Up, Right]),
        (Down, vec![Down, Up, Up]),
        (Down, vec![Left, Left, Right, Left, Right]),
        (Down, vec![Left, Left, Up, Right, Left]),
        (Down, vec![Left, Up, Left, Right, Up]),
        (Down, vec![Right, Right, Left, Right, Left]),
        (Down, vec![Right, Right, Up, Left, Right]),
        (Down, vec![Right, Up, Right, Left, Up]),
        (Left, vec![Down, Right, Left, Right]),
        (Left, vec![Down, Up, Right, Left]),
        (Left, vec![Left, Left, Right, Up]),
        (Left, vec![Right, Left]),
        (Left, vec![Right, Right, Up, Up]),
        (Right, vec![Down, Left, Right, Left]),
        (Right, vec![Down, Up, Left, Right]),
        (Right, vec![Left, Left, Up, Up]),
        (Right, vec![Left, Right]),
        (Right, vec![Right, Right, Left, Up]),
        (Up, vec![Down, Left, Up]),
        (Up, vec![Down, Right, Up]),
        (Up, vec![Left, Up, Right]),
        (Up, vec![Left, Up, Up, Right]),
        (Up, vec![Right, Up, Left]),
        (Up, vec![Right, Up, Up, Left]),
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

impl AxesBounds<usize> for CubeNet {
    fn vertical_bounds(&self) -> Range<usize> {
        self.faces.vertical_bounds()
    }

    fn horizontal_bounds(&self) -> Range<usize> {
        self.faces.horizontal_bounds()
    }
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

impl fmt::Display for CubeNet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", describe_net(&self.faces))
    }
}

pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp
    }
    a
}

fn map_adjacency<T>(net: &Net<T>) -> Edges {
    let mut frontier: Vec<Location> = Vec::new();
    let mut reached: HashSet<Location> = HashSet::new();
    let mut edges = Edges::new();

    if let Some(start) = net
        .row_major_locations()
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
        let inner_size = gcd(inner.height(), inner.width());

        let face_size = Size::square(inner_size);
        let face_area = face_size.area();

        let net_size = NET_DIMS * face_size;
        let mut faces: Vec<Option<Face>> = Vec::with_capacity(net_size.area());

        for top_left in NET_DIMS
            .row_major_locations()
            .map(|start| start * face_size + inner.top_left)
        {
            if !net.contains_some(&top_left) {
                faces.push(None);
                continue;
            }

            let bottom_right = top_left + face_size - Size::square(1);
            let face_bounds = Extents::new(top_left, bottom_right);
            let mut face_values: Vec<Location> = Vec::with_capacity(face_area);

            for location in face_bounds.row_major_locations() {
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

        Ok(Self {
            size: inner_size,
            faces,
            edges,
        })
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use geometry::index_for_location;

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
            .row_major_locations()
            .map(|location| net.get_mut(&location).unwrap().take())
            .collect();

        Net::try_from((inner.size(), values)).unwrap()
    }

    fn transpose_net<T: std::fmt::Debug + Clone>(net: Net<T>) -> Net<T> {
        let size = net.size().transpose();
        let mut values = vec![None; size.area()];

        for (location, value) in net {
            values[index_for_location(&location.transpose(), size.width())] = value;
        }

        Net::try_from((size, values)).unwrap()
    }

    fn flip_net<T>(mut net: Net<T>) -> Net<T> {
        for row in net.row_groups().take(net.height() / 2) {
            for location in row {
                net.swap(
                    &location,
                    &Location::new(location.x, net.bottom() - location.y),
                );
            }
        }
        net
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
                    net = transpose_net(net);
                    variations.push((shape, trim_net(net.clone())));
                    net = flip_net(net);
                }

                variations.into_iter()
            })
    }

    /// This generates every possible polyhedral net that will fit in a 5x5 grid. Obviously this
    /// will include the valid cube nets as well, so they will have to be filtered out later.

    fn possible_nets() -> impl Iterator<Item = Net<usize>> {
        let n = NET_DIMS.area();
        (0..n).combinations(FACES_LEN).map(move |positions| {
            let mut values = vec![None; n];

            for (j, i) in positions.into_iter().enumerate() {
                values[i] = Some(j);
            }

            Net::try_from((NET_DIMS, values)).unwrap()
        })
    }

    fn invalid_cube_nets() -> impl Iterator<Item = Net<usize>> {
        let valid_cube_nets: HashSet<NetKey> = valid_cube_nets()
            .map(|(_, net)| key_from_net(&trim_net(net)))
            .collect();

        possible_nets().filter_map(move |net| {
            let net = trim_net(net);
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
