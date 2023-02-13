use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};
use std::hash::Hash;

pub fn shortest_paths_from<'a, T>(
    neighbors: &HashMap<&'a T, HashSet<&'a T>>,
    start: &'a T,
) -> HashMap<&'a T, Vec<&'a T>>
where
    T: Hash + Eq + ?Sized,
{
    let mut frontier = VecDeque::new();
    frontier.push_back(start);

    let mut came_from = HashMap::new();

    while let Some(current) = frontier.pop_front() {
        for &neighbor in &neighbors[current] {
            if let Entry::Vacant(entry) = came_from.entry(neighbor) {
                entry.insert(current);
                frontier.push_back(neighbor);
            }
        }
    }

    let mut paths_to = HashMap::new();

    for &end in neighbors.keys() {
        if start == end {
            continue;
        }

        let mut current = end;
        let mut path = Vec::new();

        while current != start {
            path.push(current);
            current = came_from.get(current).unwrap();
        }

        path.reverse();

        paths_to.insert(end, path);
    }

    paths_to
}
