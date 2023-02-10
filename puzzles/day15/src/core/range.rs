use std::ops::RangeInclusive;

pub fn ranges_overlap(a: &RangeInclusive<isize>, b: &RangeInclusive<isize>) -> bool {
    a.contains(&(b.start() - 1))
        || a.contains(&(b.end() + 1))
        || b.contains(&(a.start() - 1))
        || b.contains(&(a.end() + 1))
}

pub fn merge_range(a: &RangeInclusive<isize>, b: &RangeInclusive<isize>) -> RangeInclusive<isize> {
    *a.start().min(b.start())..=(*a.end().max(b.end()))
}

pub fn coalesce_ranges(mut old: Vec<RangeInclusive<isize>>) -> Vec<RangeInclusive<isize>> {
    if old.is_empty() {
        return old;
    }

    let mut new: Vec<_> = Vec::new();

    old.sort_by_key(|range| *range.start());
    old.reverse();

    let mut prev = old.pop().unwrap();

    while !old.is_empty() {
        let next = old.pop().unwrap();

        if ranges_overlap(&prev, &next) {
            prev = merge_range(&prev, &next);
        } else {
            new.push(prev);
            prev = next;
        }
    }

    new.push(prev);

    new
}
