use std::collections::HashMap;
use std::convert::TryFrom;

use crate::core::{manhattan_distance, Point, Rectangle, TaxicabCircle};

use anyhow::Context;

#[derive(Debug, Default, Clone)]
pub struct SensorGrid(HashMap<Point, Point>);

impl SensorGrid {
    pub fn extents(&self) -> Rectangle {
        let mut top_left = Point::default();
        let mut bottom_right = Point::default();

        for (sensor, beacon) in self.0.iter() {
            let distance = manhattan_distance(sensor, beacon) as isize;

            let left = sensor.x - distance;
            let right = sensor.x + distance;

            let top = sensor.y - distance;
            let bottom = sensor.y + distance;

            if left < top_left.x {
                top_left.x = left;
            }

            if right > bottom_right.x {
                bottom_right.x = right;
            }

            if top < top_left.y {
                top_left.y = top;
            }

            if bottom > bottom_right.y {
                bottom_right.y = bottom;
            }
        }

        Rectangle {
            top_left,
            bottom_right,
        }
    }

    pub fn sensors(&self) -> impl Iterator<Item = &Point> {
        self.0.keys()
    }

    pub fn beacons(&self) -> impl Iterator<Item = &Point> {
        self.0.values()
    }

    pub fn taxicab_circles(&self) -> impl Iterator<Item = TaxicabCircle> + '_ {
        self.0
            .iter()
            .map(|(sensor, beacon)| TaxicabCircle::from_points(*sensor, beacon))
    }
}

impl TryFrom<&str> for SensorGrid {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        fn ensure_prefix<'a>(s: &'a str, prefix: &'a str) -> anyhow::Result<&'a str> {
            s.trim()
                .strip_prefix(prefix)
                .with_context(|| format!("expected string to start with: {:?}", prefix))
        }

        fn parse_point(s: &str) -> anyhow::Result<Point> {
            s.trim()
                .try_into()
                .with_context(|| format!("invalid point: {:?}", s))
        }

        fn parse_sensor_and_beacon(s: &str) -> anyhow::Result<(Point, Point)> {
            let mut halves = s.splitn(2, ":");

            let sensor = parse_point(ensure_prefix(
                halves.next().context("missing sensor")?,
                "Sensor at ",
            )?)?;

            let beacon = parse_point(ensure_prefix(
                halves.next().context("missing beacon")?,
                "closest beacon is at ",
            )?)?;

            Ok((sensor, beacon))
        }

        let map = s
            .lines()
            .enumerate()
            .map(|(i, s)| {
                parse_sensor_and_beacon(s).with_context(|| format!("line number {}", i + 1))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(Self(map))
    }
}
