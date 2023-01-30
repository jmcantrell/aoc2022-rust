use std::collections::HashMap;

use anyhow::Context;

use crate::core::{Expression, Job, Name, Operation, Value};

#[derive(Debug, Clone)]
pub struct MonkeyTroop<'a> {
    graph: HashMap<Name<'a>, Job<'a>>,
}

impl<'a> MonkeyTroop<'a> {
    pub fn eval(&'a self) -> Value {
        fn recurse(graph: &HashMap<Name, Job>, name: &str) -> Value {
            match graph[name] {
                Job::Value(value) => value,
                Job::Operation(operation, name1, name2) => {
                    let result1 = recurse(graph, name1);
                    let result2 = recurse(graph, name2);
                    match operation {
                        Operation::Add => result1 + result2,
                        Operation::Subtract => result1 - result2,
                        Operation::Multiply => result1 * result2,
                        Operation::Divide => result1 / result2,
                    }
                }
            }
        }

        recurse(&self.graph, "root")
    }

    pub fn eval_variable(&'a self) -> Value {
        fn recurse<'a>(graph: &HashMap<Name, Job>, name: Name<'a>) -> Expression {
            if name == "humn" {
                Expression::Function(Box::from(|value| value))
            } else {
                match graph[name] {
                    Job::Value(value) => Expression::Value(value),
                    Job::Operation(operation, name1, name2) => {
                        let result1 = recurse(graph, name1);
                        let result2 = recurse(graph, name2);

                        match (result1, result2) {
                            (Expression::Value(value1), Expression::Value(value2)) => {
                                Expression::Value(match operation {
                                    Operation::Add => value1 + value2,
                                    Operation::Subtract => value1 - value2,
                                    Operation::Multiply => value1 * value2,
                                    Operation::Divide => value1 / value2,
                                })
                            }
                            (Expression::Value(value), Expression::Function(function)) => {
                                Expression::Function(Box::from(move |equals| {
                                    function(match operation {
                                        Operation::Add => equals - value,
                                        Operation::Subtract => value - equals,
                                        Operation::Multiply => equals / value,
                                        Operation::Divide => value / equals,
                                    })
                                }))
                            }
                            (Expression::Function(function), Expression::Value(value)) => {
                                Expression::Function(Box::from(move |equals| {
                                    function(match operation {
                                        Operation::Add => equals - value,
                                        Operation::Subtract => value + equals,
                                        Operation::Multiply => equals / value,
                                        Operation::Divide => value * equals,
                                    })
                                }))
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }

        match self.graph["root"] {
            Job::Operation(_, name1, name2) => {
                let result1 = recurse(&self.graph, name1);
                let result2 = recurse(&self.graph, name2);

                match (result1, result2) {
                    (Expression::Value(value), Expression::Function(function)) => function(value),
                    (Expression::Function(function), Expression::Value(value)) => function(value),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}

impl<'a> FromIterator<(Name<'a>, Job<'a>)> for MonkeyTroop<'a> {
    fn from_iter<I: IntoIterator<Item = (Name<'a>, Job<'a>)>>(iter: I) -> Self {
        Self {
            graph: iter.into_iter().collect(),
        }
    }
}

impl<'a> TryFrom<&'a str> for MonkeyTroop<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        fn parse_name_and_job(s: &str) -> anyhow::Result<(Name, Job)> {
            let mut split = s.splitn(2, ": ");
            let name = split.next().context("missing name")?;
            let job: Job = split.next().context("missing job")?.try_into()?;
            Ok((name, job))
        }

        s.lines()
            .enumerate()
            .map(|(i, s)| parse_name_and_job(s).with_context(|| format!("line number {}", i + 1)))
            .collect::<Result<_, _>>()
            .into()
    }
}
