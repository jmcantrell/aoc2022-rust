use std::convert::TryFrom;

use anyhow::{anyhow, ensure, Context};

const CRATE_WIDTH: usize = 3;

pub type Crate = char;
pub type Stack = Vec<Crate>;

#[derive(Debug, Clone)]
pub struct Stacks(pub Vec<Stack>);

impl Stacks {
    pub fn pick_up(&mut self, from: usize) -> Option<Crate> {
        self.0[from].pop()
    }

    pub fn put_down(&mut self, to: usize, c: Crate) {
        self.0[to].push(c);
    }

    pub fn top(&self) -> Vec<Option<&Crate>> {
        self.0.iter().map(|stack| stack.last()).collect()
    }
}

impl TryFrom<&str> for Stacks {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut lines = s.lines().rev();

        let num_stacks: usize = lines
            .next()
            .context("missing header")?
            .split_whitespace()
            .count();

        let expected_line_length = num_stacks * (CRATE_WIDTH + 1) - 1;

        let rows = lines
            .enumerate()
            .map(|(i, s)| -> anyhow::Result<Vec<Option<(usize, char)>>> {
                ensure!(
                    s.len() == expected_line_length,
                    "expected line length to be {} characters, but was {} instead",
                    expected_line_length,
                    s.len()
                );

                let chars: Vec<_> = s.chars().collect();

                (0..num_stacks)
                    .map(|i| -> anyhow::Result<Option<(usize, char)>> {
                        let column = i * (CRATE_WIDTH + 1) + 1;

                        let mark = chars[column];
                        let open = chars[column - 1];
                        let close = chars[column + 1];

                        if open != '[' {
                            Ok(None)
                        } else if close != ']' {
                            Err(anyhow!(
                                "invalid right wall for crate number {}: {:?}",
                                i + 1,
                                close
                            ))
                        } else if mark.is_whitespace() {
                            Err(anyhow!("crate number {} is missing its mark", i + 1))
                        } else {
                            Ok(Some((i, mark)))
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .with_context(|| format!("row number {}", i + 1))
            })
            .collect::<Result<Vec<_>, _>>()
            .context("unable to parse stacks")?;

        let mut stacks: Vec<Vec<Crate>> = vec![Default::default(); num_stacks];

        rows.into_iter().for_each(|row| {
            row.into_iter().flatten().for_each(|(i, mark)| {
                stacks[i].push(mark);
            });
        });

        Ok(Self(stacks))
    }
}
