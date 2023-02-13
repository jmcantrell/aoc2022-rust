use std::collections::HashMap;
use std::ops::Add;

use anyhow::Context;
use lazy_static::lazy_static;
use regex::Regex;

use crate::core::{Resource, ResourceContainer, ResourceCount, ResourceMap, ResourceTally};

const INITIAL_POPULATION: ResourceContainer<ResourceCount> = [1, 0, 0, 0];

type Minutes = u16;

fn div_ceil(dividend: ResourceCount, divisor: ResourceCount) -> ResourceCount {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    quotient + if remainder == 0 { 0 } else { 1 }
}

fn buildable(
    cost: &ResourceTally,
    population: &ResourceTally,
    resources: &ResourceTally,
    minutes_remaining: Minutes,
) -> Option<Minutes> {
    if minutes_remaining == 0 {
        return None;
    }

    let mut max_minutes_required = 1;

    for (resource, &amount) in cost.iter_non_zero() {
        let per_minute = population[resource];

        if per_minute == 0 {
            return None;
        }

        let amount_required = amount.saturating_sub(resources[resource]);

        if amount_required == 0 {
            continue;
        }

        let minutes_required = div_ceil(amount_required, per_minute) + 1;

        if minutes_required >= minutes_remaining {
            return None;
        }

        max_minutes_required = max_minutes_required.max(minutes_required);
    }

    Some(max_minutes_required)
}

pub type Identifier = u8;
pub type RobotCost = ResourceMap<ResourceTally>;

#[derive(Debug, Clone)]
pub struct Blueprint {
    pub id: Identifier,
    pub robots: RobotCost,
}

impl Blueprint {
    pub fn max_geodes_collectable(&self, minutes: Minutes) -> ResourceCount {
        fn recurse(
            blueprint: &Blueprint,
            cache: &mut HashMap<State, ResourceCount>,
            best: &mut ResourceCount,
            state: State,
        ) -> ResourceCount {
            let current = *state.resources.geode();

            if state.minutes_remaining == 0 {
                return current;
            }

            if let Some(value) = cache.get(&state) {
                return *value;
            }

            let n = state.minutes_remaining;
            let will_collect = *state.population.geode() * n;
            let possibly_collect = n * (n - 1) / 2;

            if current + will_collect + possibly_collect <= *best {
                return current;
            }

            let value = state
                .branches(blueprint)
                .map(|branch| recurse(blueprint, cache, best, state + branch))
                .max()
                .unwrap_or_else(|| *state.finish().resources.geode());

            cache.insert(state, value);

            *best = std::cmp::max(*best, value);

            value
        }

        let mut cache = HashMap::new();
        let mut best = 0;

        recurse(self, &mut cache, &mut best, State::new(minutes))
    }
}

impl TryFrom<&str> for Blueprint {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        fn parse_resource_cost(s: &str) -> anyhow::Result<(Resource, ResourceCount)> {
            lazy_static! {
                static ref RE: Regex = r"^(\d+) (\S+)$".parse().unwrap();
            }

            let captures = RE
                .captures(s)
                .with_context(|| format!("invalid resource cost: {s:?}"))?;

            let cost: ResourceCount = captures[1]
                .parse()
                .with_context(|| format!("invalid cost: {:?}", &captures[0]))?;

            let resource: Resource = captures[2].try_into()?;

            Ok((resource, cost))
        }

        fn parse_robot_and_cost(s: &str) -> anyhow::Result<(Resource, ResourceTally)> {
            lazy_static! {
                static ref RE: Regex = r"^Each (\S+) robot costs (.*)$".parse().unwrap();
            }

            let captures = RE
                .captures(s)
                .with_context(|| format!("invalid robot specification: {s:?}"))?;

            let resource: Resource = captures[1].try_into()?;

            let costs: ResourceTally = captures[2]
                .split(" and ")
                .enumerate()
                .map(|(i, s)| {
                    parse_resource_cost(s).with_context(|| format!("cost number {}", i + 1))
                })
                .collect::<Result<HashMap<_, _>, _>>()?
                .into();

            Ok((resource, costs))
        }

        lazy_static! {
            static ref RE: Regex = r"^Blueprint (\d+): (.*)$".parse().unwrap();
        }

        let captures = RE
            .captures(s)
            .with_context(|| format!("invalid blueprint: {s:?}"))?;

        let id: Identifier = captures[1].parse()?;

        let robots: RobotCost = captures[2]
            .trim_end_matches('.')
            .split(". ")
            .enumerate()
            .map(|(i, s)| {
                parse_robot_and_cost(s).with_context(|| format!("robot number {}", i + 1))
            })
            .collect::<Result<HashMap<_, _>, _>>()?
            .into();

        Ok(Blueprint { id, robots })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pub minutes_remaining: Minutes,
    pub population: ResourceTally,
    pub resources: ResourceTally,
}

impl State {
    fn new(minutes_remaining: Minutes) -> Self {
        Self {
            minutes_remaining,
            population: INITIAL_POPULATION.into(),
            resources: Default::default(),
        }
    }

    fn accumulate(mut self, minutes: Minutes) -> Self {
        self.resources += self.population * minutes;
        self
    }

    fn finish(self) -> Self {
        self.accumulate(self.minutes_remaining)
    }

    fn branches<'a>(&'a self, blueprint: &'a Blueprint) -> impl Iterator<Item = Branch> + '_ {
        blueprint.robots.iter().filter_map(|(&robot, &cost)| {
            buildable(
                &cost,
                &self.population,
                &self.resources,
                self.minutes_remaining,
            )
            .map(|minutes_required| Branch {
                robot,
                cost,
                minutes_required,
            })
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Branch {
    robot: Resource,
    cost: ResourceTally,
    minutes_required: Minutes,
}

impl Add<Branch> for State {
    type Output = Self;

    fn add(self, branch: Branch) -> Self {
        let mut result = self.accumulate(branch.minutes_required);
        result.resources -= branch.cost;
        result.population[&branch.robot] += 1;
        result.minutes_remaining -= branch.minutes_required;
        result
    }
}

#[derive(Debug, Clone, Default)]
struct Timeline(Vec<(Branch, State)>);

impl Add<(Branch, State)> for Timeline {
    type Output = Self;

    fn add(mut self, item: (Branch, State)) -> Self {
        self.0.push(item);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_blueprint1() -> Blueprint {
        Blueprint {
            id: 1,
            robots: [
                [4, 0, 0, 0].into(),
                [2, 0, 0, 0].into(),
                [3, 14, 0, 0].into(),
                [2, 0, 7, 0].into(),
            ]
            .into(),
        }
    }

    #[test]
    fn buildable_only_if_enough_time() {
        let blueprint = example_blueprint1();

        macro_rules! assert_buildable {
            ($population:expr, $resources:expr, $expected:expr, $minutes:expr) => {
                assert_eq!(
                    crate::core::RESOURCES.map(|robot| {
                        buildable(
                            &blueprint.robots[&robot],
                            &$population.into(),
                            &$resources.into(),
                            $minutes,
                        )
                    }),
                    $expected
                );
            };
        }

        // If there's no time left, nothing can be built, regardless of the population.
        assert_buildable!([0, 0, 0, 0], [0, 0, 0, 0], [None, None, None, None], 0);
        assert_buildable!([1, 2, 3, 4], [0, 0, 0, 0], [None, None, None, None], 0);

        // Nothing can be built if there's plenty of time, but nothing to collect resources.
        assert_buildable!([0, 0, 0, 0], [0, 0, 0, 0], [None, None, None, None], 20);

        assert_buildable!(
            [1, 2, 3, 4],
            [0, 0, 0, 0],
            [None, Some(3), None, Some(4)],
            5
        );
    }

    #[test]
    fn buildable_depending_on_resources() {
        let blueprint = example_blueprint1();

        macro_rules! assert_buildable {
            ($population:expr, $resources:expr, $expected:expr) => {
                assert_eq!(
                    crate::core::RESOURCES.map(|robot| {
                        buildable(
                            &blueprint.robots[&robot],
                            &$population.into(),
                            &$resources.into(),
                            24,
                        )
                    }),
                    $expected
                );
            };
        }

        // Nothing can be collected if there are no robots to collect it.
        assert_buildable!([0, 0, 0, 0], [0, 0, 0, 0], [None, None, None, None]);

        // If only one ore robot exists, only robots that just require ore can be built.
        assert_buildable!([1, 0, 0, 0], [0, 0, 0, 0], [Some(5), Some(3), None, None]);

        // ... and it's proportional to the number of robots collecting ore.
        assert_buildable!([2, 0, 0, 0], [0, 0, 0, 0], [Some(3), Some(2), None, None]);

        // It's guaranteed that there will never be resources without the robots to collect it, but
        // just to reinforce that, let's ensure that the function will not even calculate it.
        assert_buildable!([0, 0, 0, 0], [100, 100, 100, 100], [None, None, None, None]);

        // If there are already enough resources to build something, it will take no time at all.
        assert_buildable!([1, 0, 0, 0], [4, 0, 0, 0], [Some(1), Some(1), None, None]);

        // ... but if there is a deficit, only the difference will count toward the build time.
        assert_buildable!(
            [1, 2, 3, 4],
            [2, 3, 4, 5],
            [Some(3), Some(1), Some(7), Some(2)]
        );
    }
}
