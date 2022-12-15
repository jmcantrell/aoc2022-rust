use std::cmp::Reverse;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::BinaryHeap;

#[derive(Debug, Default)]
pub struct Priorized<T>(T, usize);

impl<T> PartialEq for Priorized<T> {
    fn eq(&self, other: &Self) -> bool {
        self.1.eq(&other.1)
    }
}

impl<T> Eq for Priorized<T> {}

impl<T> PartialOrd for Priorized<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Priorized<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

#[derive(Debug, Clone)]
pub struct PriorityQueue<T>(BinaryHeap<Reverse<T>>);

impl<T> PriorityQueue<Priorized<T>> {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn push(&mut self, item: T, priority: usize) {
        self.0.push(Reverse(Priorized(item, priority)));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop().map(|rev| rev.0 .0)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
