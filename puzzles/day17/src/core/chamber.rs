use std::collections::HashSet;
use std::fmt;
use std::iter::{repeat, Cycle, Enumerate};
use std::ops::Add;
use std::vec::IntoIter;

use crate::core::{JetPush, Rock};

type Row = u8;
type Chunk = u32;
type Block = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SampleKey {
    block: Block,
    rock_index: usize,
    jet_push_index: usize,
}

const WIDTH: usize = 7;
const BEDROCK: Row = Row::MAX;
const CLEARANCE_ROWS: usize = 3;
const CLEARANCE_COLUMNS: usize = 2;

const CHUNK_ROW_LEN: usize = 4;
const BLOCK_ROW_LEN: usize = 8;

const ROW_LEFT_MASK: Row = 0b01000000;
const CHUNK_LEFT_MASK: Chunk = Chunk::from_be_bytes([ROW_LEFT_MASK; CHUNK_ROW_LEN]);

const ROW_RIGHT_MASK: Row = 0b00000001;
const CHUNK_RIGHT_MASK: Chunk = Chunk::from_be_bytes([ROW_RIGHT_MASK; CHUNK_ROW_LEN]);

const ROCK_PATTERN: [Rock; 5] = [
    Rock::Slab,
    Rock::Plus,
    Rock::Ell,
    Rock::Column,
    Rock::Square,
];

impl Add<JetPush> for Chunk {
    type Output = Self;

    fn add(self, jet_push: JetPush) -> Self::Output {
        match jet_push {
            JetPush::Left => self << 1,
            JetPush::Right => self >> 1,
        }
    }
}

impl JetPush {
    fn edge_mask(&self) -> Chunk {
        match self {
            Self::Left => CHUNK_LEFT_MASK,
            Self::Right => CHUNK_RIGHT_MASK,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Chamber {
    rock_pile: Vec<Row>,
    falling_rock: Option<(Chunk, usize)>,
    falling_rocks: Cycle<Enumerate<IntoIter<Chunk>>>,
    jet_pushes: Cycle<Enumerate<IntoIter<JetPush>>>,
}

impl Chamber {
    pub fn new(jet_pattern: &Vec<JetPush>) -> Self {
        let rock_offset = (Row::BITS as usize - WIDTH) + CLEARANCE_COLUMNS;

        let rock_pattern: Vec<_> = ROCK_PATTERN
            .into_iter()
            .map(|rock| Chunk::from_le_bytes(rock.bytes()) >> rock_offset)
            .collect();

        assert!(jet_pattern.len() > 0, "jet pattern empty");
        assert!(rock_pattern.len() > 0, "rock pattern empty");

        Self {
            rock_pile: vec![BEDROCK],
            falling_rock: None,
            falling_rocks: rock_pattern.into_iter().enumerate().cycle(),
            jet_pushes: jet_pattern.clone().into_iter().enumerate().cycle(),
        }
    }

    pub fn height(&self) -> usize {
        self.rock_pile.len() - 1
    }

    fn view(&self, i: usize, size: usize) -> Vec<Row> {
        self.rock_pile
            .iter()
            .chain(repeat(&0))
            .skip(i)
            .take(size)
            .cloned()
            .collect()
    }

    fn chunk(&self, i: usize) -> Chunk {
        Chunk::from_be_bytes(self.view(i, CHUNK_ROW_LEN).as_slice().try_into().unwrap())
    }

    fn block(&self, i: usize) -> Block {
        Block::from_be_bytes(self.view(i, BLOCK_ROW_LEN).as_slice().try_into().unwrap())
    }

    fn top_block(&self) -> Block {
        self.block(self.rock_pile.len().saturating_sub(BLOCK_ROW_LEN))
    }

    fn add_rock(&mut self, i: usize, rock: Chunk) {
        let bound = i + CHUNK_ROW_LEN;

        while self.rock_pile.len() < bound {
            self.rock_pile.push(0);
        }

        self.rock_pile
            .iter_mut()
            .skip(i)
            .zip(rock.to_be_bytes().into_iter())
            .for_each(|(row, layer)| *row |= layer);

        while let Some(0) = self.rock_pile.last() {
            self.rock_pile.pop().unwrap();
        }
    }

    pub fn drop_rock(&mut self) -> SampleKey {
        let (rock_index, mut rock) = self.falling_rocks.next().unwrap();

        let mut rock_bottom = self.rock_pile.len() + CLEARANCE_ROWS;

        self.falling_rock = Some((rock, rock_bottom));
        // println!("new rock:\n{self}");

        let jet_push_index = loop {
            let (jet_push_index, jet_push) = self.jet_pushes.next().unwrap();

            if rock & jet_push.edge_mask() == 0 {
                let pushed_rock = rock + jet_push;

                if pushed_rock & self.chunk(rock_bottom) == 0 {
                    rock = pushed_rock;
                }
            }

            self.falling_rock = Some((rock, rock_bottom));
            // println!("pushed {:?}:\n{self}", jet_push);

            if rock & self.chunk(rock_bottom - 1) == 0 {
                rock_bottom -= 1;
                self.falling_rock = Some((rock, rock_bottom));
                // println!("dropped:\n{self}");
            } else {
                break jet_push_index;
            }
        };

        self.falling_rock = None;
        self.add_rock(rock_bottom, rock);
        // println!("rock settled:\n{self}");

        SampleKey {
            block: self.top_block(),
            rock_index,
            jet_push_index,
        }
    }
}

impl fmt::Display for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn bits(row: Row) -> impl Iterator<Item = bool> {
            (0..WIDTH)
                .into_iter()
                .rev()
                .map(move |i| row & (1 << i) != 0)
        }

        let falling_rock: HashSet<_> =
            self.falling_rock
                .map_or_else(Default::default, |(rock, rock_height)| {
                    rock.to_be_bytes()
                        .into_iter()
                        .enumerate()
                        .map(|(i, layer)| {
                            bits(layer).enumerate().filter_map(move |(column, set)| {
                                if set {
                                    Some((rock_height + i, column))
                                } else {
                                    None
                                }
                            })
                        })
                        .flatten()
                        .collect()
                });

        let max_height = self.rock_pile.len().max(
            falling_rock
                .iter()
                .map(|&(row, _)| row)
                .max()
                .unwrap_or_default(),
        );

        for i in (0..max_height).rev() {
            let row = self.rock_pile.get(i).cloned().unwrap_or_default();

            for (j, set) in bits(row).enumerate() {
                let c = if falling_rock.contains(&(i, j)) {
                    '@'
                } else if set {
                    '#'
                } else {
                    '.'
                };

                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
