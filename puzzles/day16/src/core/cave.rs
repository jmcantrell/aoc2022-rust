use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

use anyhow::Context;
use regex::Regex;

use crate::core::shortest_paths_from;

const START: &'static str = "AA";

#[derive(Debug, Clone)]
pub struct RoomGraph<'a> {
    pub tunnels: HashMap<&'a str, HashSet<&'a str>>,
    pub flow_rates: HashMap<&'a str, usize>,
    pub seconds_to_open: HashMap<(&'a str, &'a str), usize>,
}

impl<'a> RoomGraph<'a> {
    pub fn traverse_possible_paths<F>(&self, valves: &HashSet<&str>, seconds: usize, mut visit: F)
    where
        F: FnMut(&[&str], &HashSet<&str>, usize),
    {
        let mut timelines = vec![(seconds, valves.clone(), vec![START], 0, 0)];

        while let Some((seconds, choices, path, ppm, released)) = timelines.pop() {
            visit(&path, &choices, released + ppm * seconds);

            for &next_location in &choices {
                let last_location = *path.last().unwrap();
                let seconds_required = self.seconds_to_open[&(last_location, next_location)];

                if seconds <= seconds_required {
                    continue;
                }

                let mut choices = choices.clone();
                choices.remove(next_location);

                let mut path = path.clone();
                path.push(next_location);

                let seconds = seconds - seconds_required;
                let released = released + seconds_required * ppm;
                let ppm = ppm + self.flow_rates[next_location];

                timelines.push((seconds, choices, path, ppm, released));
            }
        }
    }
}

impl<'a> TryFrom<&'a str> for RoomGraph<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        fn parse_line<'a>(
            s: &'a str,
            re: &Regex,
        ) -> anyhow::Result<(&'a str, usize, HashSet<&'a str>)> {
            let captures = re
                .captures(s)
                .with_context(|| format!("did not match regex: {:?}", re))?;

            let name = captures.get(1).context("missing name")?.as_str();

            let flow_rate: usize = captures
                .get(2)
                .context("missing flow rate")?
                .as_str()
                .parse()?;

            let neighbors = captures
                .get(3)
                .context("missing tunnels")?
                .as_str()
                .split(",")
                .map(|s| s.trim())
                .collect::<HashSet<_>>();

            Ok((name, flow_rate, neighbors))
        }

        let mut tunnels = HashMap::new();
        let mut flow_rates = HashMap::new();

        let re = Regex::new(r"^Valve (\S+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
            .context("invalid regex")?;

        for (i, s) in s.lines().enumerate() {
            let (name, flow_rate, neighbors) =
                parse_line(s, &re).with_context(|| format!("line number {}", i + 1))?;

            tunnels.insert(name, neighbors);
            if flow_rate > 0 {
                flow_rates.insert(name, flow_rate);
            }
        }

        let mut seconds_to_open = HashMap::new();

        for &start in flow_rates.keys().chain(std::iter::once(&START)) {
            for (end, path) in shortest_paths_from(&tunnels, start) {
                seconds_to_open.insert((start, end), path.len() + 1);
            }
        }

        Ok(Self {
            tunnels,
            flow_rates,
            seconds_to_open,
        })
    }
}
