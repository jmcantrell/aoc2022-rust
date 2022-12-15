use std::convert::TryFrom;
use std::ops::RangeInclusive;

use anyhow::{anyhow, ensure, Context};

use crate::core::{BreadCrumbs, Cell, Grid, Location, Path};

pub type Height = usize;
pub type HeightCell<'a> = Cell<'a, Height>;

#[derive(Debug, Clone)]
pub struct HeightMap {
    grid: Grid<Height>,
    pub start: Location,
    pub end: Location,
}

impl HeightMap {
    fn breadcrumbs(&self) -> BreadCrumbs {
        BreadCrumbs::from_grid(&self.grid, self.end, |(_, a), (_, b)| {
            a < b || a.abs_diff(*b) <= 1
        })
    }

    pub fn find_shortest_path(&self) -> Option<Path> {
        self.breadcrumbs().path(self.start)
    }

    pub fn find_alternate_paths(&self) -> impl Iterator<Item = Path> + '_ {
        let breadcrumbs = self.breadcrumbs();

        self.grid
            .cells()
            .filter(|(_, &height)| height == 0)
            .filter_map(move |(start, _)| breadcrumbs.path(start))
    }
}

const START: char = 'S';
const END: char = 'E';
const HEIGHT_RANGE: RangeInclusive<char> = 'a'..='z';

impl TryFrom<&str> for HeightMap {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut lines = s.lines().peekable();

        let width = lines.peek().context("no lines")?.len();

        let mut start = Default::default();
        let mut end = Default::default();

        fn parse_height(c: char) -> anyhow::Result<Height> {
            if HEIGHT_RANGE.contains(&c) {
                Ok(c as usize - *HEIGHT_RANGE.start() as usize)
            } else {
                Err(anyhow!("unrecognized character: {:?}", c))
            }
        }

        let mut cells = Vec::new();

        for (i, line) in lines.enumerate() {
            ensure!(
                line.len() == width,
                "expected line number {} length to be {}, but was {}",
                i + 1,
                width,
                line.len()
            );

            for (j, c) in line.chars().enumerate() {
                let mut c = c;

                if c == START || c == END {
                    let location = Location { row: i, column: j };
                    if c == START {
                        start = location;
                        c = *HEIGHT_RANGE.start();
                    } else {
                        end = location;
                        c = *HEIGHT_RANGE.end();
                    }
                }

                let height = parse_height(c)
                    .with_context(|| format!("line number {}, row number {}", i + 1, j + 1))?;

                cells.push(height);
            }
        }

        let grid = Grid::try_from_cells(cells, width).context("invalid grid")?;

        Ok(Self { grid, start, end })
    }
}
