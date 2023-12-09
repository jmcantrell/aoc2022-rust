use std::collections::HashMap;
use std::fmt;
use std::ops::Range;

use super::{Direction, Location, Map};

use Direction::*;

pub type MapIterItem = HashMap<Location, Vec<Direction>>;

#[derive(Debug, Clone)]
pub struct MapIter<'a> {
    map: &'a Map,
    next: Option<MapIterItem>,
    inner: Range<usize>,
}

impl<'a> MapIter<'a> {
    pub fn new(map: &'a Map) -> Self {
        let inner_height = map.grid.nrows() - 2;
        let inner_width = map.grid.ncols() - 2;

        let start = map
            .blizzards
            .clone()
            .into_iter()
            .map(|(location, direction)| (location, vec![direction]))
            .collect();

        Self {
            map,
            next: Some(start),
            inner: 1..(inner_height * inner_width),
        }
    }
}

impl<'a> Iterator for MapIter<'a> {
    type Item = MapIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next.take()?;

        self.next = self.inner.next().map(|_| {
            let mut next = MapIterItem::new();

            let (height, width) = self.map.grid.shape();

            let left = 1;
            let right = width - 2;
            let top = 1;
            let bottom = height - 2;

            for (from, blizzards) in current.iter() {
                let (row, column) = from.into_inner();

                for direction in blizzards.iter() {
                    let row = match direction {
                        Up => {
                            if row == top {
                                bottom
                            } else {
                                row - 1
                            }
                        }
                        Down => {
                            if row == bottom {
                                top
                            } else {
                                row + 1
                            }
                        }
                        _ => row,
                    };

                    let column = match direction {
                        Left => {
                            if column == left {
                                right
                            } else {
                                column - 1
                            }
                        }
                        Right => {
                            if column == right {
                                left
                            } else {
                                column + 1
                            }
                        }
                        _ => column,
                    };

                    let to = (row, column).into();

                    next.entry(to).or_default().push(*direction);
                }
            }

            next
        });

        Some(current)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a> ExactSizeIterator for MapIter<'a> {}

impl<'a> fmt::Display for MapIter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(blizzards) = self.next.as_ref() {
            for row in 0..self.map.grid.nrows() {
                for column in 0..self.map.grid.ncols() {
                    let tile = self.map.grid[(row, column)];
                    write!(
                        f,
                        "{}",
                        if let Some(directions) = blizzards.get(&(row, column).into()) {
                            let n = directions.len();
                            if n == 1 {
                                directions[0].into()
                            } else if n <= 9 {
                                char::from_digit(n as u32, 10).unwrap()
                            } else {
                                '+'
                            }
                        } else {
                            tile.into()
                        }
                    )?;
                }
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::hash::Hash;

    use super::*;

    fn freq<T: Eq + Hash>(items: impl Iterator<Item = T>) -> HashMap<T, usize> {
        let mut result = HashMap::new();

        for item in items {
            *result.entry(item).or_default() += 1;
        }

        result
    }

    fn normalize_map_iter_item(item: MapIterItem) -> HashMap<Location, HashMap<Direction, usize>> {
        item.into_iter()
            .map(|(location, directions)| (location, freq(directions.into_iter())))
            .collect()
    }

    fn assert_iter_item(actual: MapIterItem, expected: MapIterItem) {
        assert_eq!(
            normalize_map_iter_item(actual),
            normalize_map_iter_item(expected)
        );
    }

    #[test]
    fn test_example1() {
        let map = Map::try_from(
            "\
            #.#####\n\
            #.....#\n\
            #>....#\n\
            #.....#\n\
            #...v.#\n\
            #.....#\n\
            #####.#\n\
            ",
        )
        .unwrap();

        let mut iter = MapIter::new(&map);
        macro_rules! assert_next {
            ($map:expr, $item:expr) => {
                assert_eq!(iter.to_string(), $map);
                assert_iter_item(
                    iter.next().unwrap(),
                    $item
                        .into_iter()
                        .map(|(pair, directions)| (pair.into(), directions.into_iter().collect()))
                        .collect(),
                );
            };
        }

        assert_next!(
            "\
            #.#####\n\
            #.....#\n\
            #>....#\n\
            #.....#\n\
            #...v.#\n\
            #.....#\n\
            #####.#\n\
            ",
            [((2, 1), [Right]), ((4, 4), [Down])]
        );

        assert_next!(
            "\
            #.#####\n\
            #.....#\n\
            #.>...#\n\
            #.....#\n\
            #.....#\n\
            #...v.#\n\
            #####.#\n\
            ",
            [((2, 2), [Right]), ((5, 4), [Down])]
        );

        assert_next!(
            "\
            #.#####\n\
            #...v.#\n\
            #..>..#\n\
            #.....#\n\
            #.....#\n\
            #.....#\n\
            #####.#\n\
            ",
            [((2, 3), [Right]), ((1, 4), [Down])]
        );

        assert_next!(
            "\
            #.#####\n\
            #.....#\n\
            #...2.#\n\
            #.....#\n\
            #.....#\n\
            #.....#\n\
            #####.#\n\
            ",
            [((2, 4), [Down, Right])]
        );

        assert_next!(
            "\
            #.#####\n\
            #.....#\n\
            #....>#\n\
            #...v.#\n\
            #.....#\n\
            #.....#\n\
            #####.#\n\
            ",
            [((2, 5), [Right]), ((3, 4), [Down])]
        );

        assert_next!(
            "\
            #.#####\n\
            #.....#\n\
            #>....#\n\
            #.....#\n\
            #...v.#\n\
            #.....#\n\
            #####.#\n\
            ",
            [((2, 1), [Right]), ((4, 4), [Down])]
        );
    }

    #[test]
    fn test_num_iterations_equals_inner_area() {
        let map = Map::try_from(
            "\
            #.##\n\
            #>v#\n\
            #..#\n\
            #^<#\n\
            ##.#\n\
            ",
        )
        .unwrap();

        assert_eq!(MapIter::new(&map).count(), 6);
    }
}
