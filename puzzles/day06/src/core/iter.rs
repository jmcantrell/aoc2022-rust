use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

pub fn find_unique_window<I, T>(iter: I, size: usize) -> Option<usize>
where
    I: Iterator<Item = T>,
    T: Clone + Eq + Hash,
{
    let mut window: VecDeque<T> = Default::default();
    let mut count: usize = 0;

    for c in iter {
        window.push_back(c);
        count += 1;

        if window.len() > size {
            window.pop_front().unwrap();
        }

        if window.len() == size {
            let set: HashSet<T> = HashSet::from_iter(window.iter().cloned());
            if set.len() == size {
                return Some(count);
            }
        }
    }

    None
}
