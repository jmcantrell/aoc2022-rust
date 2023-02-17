use std::collections::VecDeque;

use super::{Direction, Grid, Location, Map, Tile};

use Direction::*;
use Tile::*;

const DIRECTIONS: [Direction; 4] = [Up, Down, Left, Right];

pub fn find_fastest_time(map: &Map, waypoints: &[Location]) -> usize {
    if waypoints.is_empty() {
        return 0;
    }

    let (height, width) = map.grid.shape();

    let moments: Vec<_> = map.iter().collect();

    let mut waypoints = waypoints.to_owned();
    waypoints.reverse();

    let mut path = vec![waypoints.pop().unwrap()];

    while let Some(end) = waypoints.pop() {
        let start = path.pop().unwrap();

        let mut maybe_best_leg: Option<Vec<Location>> = None;

        let mut queue = VecDeque::new();
        queue.push_back(vec![start]);

        let mut memo = Grid::from_element(moments.len(), map.grid.len(), false);

        while let Some(leg) = queue.pop_front() {
            let location = *leg.last().unwrap();

            if location == end {
                if let Some(best_leg) = maybe_best_leg.as_ref() {
                    if leg.len() < best_leg.len() {
                        maybe_best_leg = Some(leg);
                    }
                } else {
                    maybe_best_leg = Some(leg);
                }
                continue;
            }

            if let Some(best_leg) = maybe_best_leg.as_ref() {
                if leg.len() >= best_leg.len() {
                    continue;
                }
            }

            let i = (path.len() + leg.len()) % moments.len();
            let next_moment = &moments[i];

            let mut choices: Vec<_> = DIRECTIONS
                .into_iter()
                .filter_map(move |direction| {
                    let adjacent = location.neighbor(&direction)?;
                    let (row, column) = adjacent.into_inner();
                    (row < height
                        && column < width
                        && map.grid[adjacent.into_inner()] == Floor
                        && !next_moment.contains_key(&adjacent))
                    .then_some(adjacent)
                })
                .collect();

            choices.sort_by_key(|adjacent| {
                adjacent.row().abs_diff(*end.row()) + adjacent.column().abs_diff(*end.column())
            });

            if !next_moment.contains_key(&location) {
                choices.push(location);
            }

            for adjacent in choices {
                let key = (i, adjacent.row() * width + adjacent.column());

                if memo[key] {
                    continue;
                }

                memo[key] = true;

                let mut leg = leg.clone();
                leg.push(adjacent);
                queue.push_back(leg);
            }
        }

        for location in maybe_best_leg.unwrap().into_iter() {
            path.push(location);
        }
    }

    path.len() - 1
}
