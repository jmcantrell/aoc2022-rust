use anyhow::Context;
use std::convert::TryFrom;
use std::fs;

const CRATE_WIDTH: usize = 3;

type Crate = char;

#[derive(Debug)]
struct StackSet(Vec<Vec<Crate>>);

impl StackSet {
    fn exec(&mut self, command: &Command) -> anyhow::Result<()> {
        match command {
            Command::Move { count, from, to } => {
                let mut buffer: Vec<Crate> = Default::default();

                for _ in 0..*count {
                    buffer.push(
                        self.0[*from - 1]
                            .pop()
                            .with_context(|| format!("Stack {} is empty", from))?,
                    );
                }

                while !buffer.is_empty() {
                    self.0[*to - 1].push(buffer.pop().unwrap());
                }
            }
        }

        Ok(())
    }

    fn top(&self) -> Vec<&Crate> {
        self.0
            .iter()
            .map(|stack| stack.last().unwrap_or(&' '))
            .collect()
    }
}

impl TryFrom<&str> for StackSet {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut lines = s.lines().rev();
        let header = lines.next().context("Header is missing")?;
        let num_stacks = header.split_whitespace().count();

        let mut stacks: Vec<Vec<Crate>> = vec![Default::default(); num_stacks];

        let expected_line_length = num_stacks * (CRATE_WIDTH + 1) - 1;

        for (line, s) in lines.enumerate() {
            if s.len() != expected_line_length {
                anyhow::bail!(
                    "Expected line length to be {}, but was {} instead",
                    expected_line_length,
                    s.len()
                );
            }

            let chars: Vec<_> = s.chars().collect();

            for i in 0..num_stacks {
                let column = i * (CRATE_WIDTH + 1) + 1;

                let mark = chars[column];
                let open = chars[column - 1];
                let close = chars[column + 1];

                if open != '[' {
                    continue;
                }

                if mark.is_whitespace() {
                    anyhow::bail!(
                        "Crate for stack {} on line {} is missing a mark",
                        i + 1,
                        line + 1
                    );
                }

                if close != ']' {
                    anyhow::bail!(
                        "Crate for stack {} on line {} missing a closing `]`",
                        i + 1,
                        line + 1
                    );
                }

                stacks[i].push(mark);
            }
        }

        Ok(Self(stacks))
    }
}

#[derive(Debug)]
enum Command {
    Move {
        count: usize,
        from: usize,
        to: usize,
    },
}

impl TryFrom<&str> for Command {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut tokens = s.split_whitespace();
        let command = tokens.next().context("Missing command")?;

        match command {
            "move" => {
                let count: usize = tokens.next().context("Missing count")?.parse()?;

                let from = tokens.next().context("Missing `from` subcommand")?;
                if from != "from" {
                    anyhow::bail!("Expected `from` but got {:?} instead", from);
                }

                let from = tokens
                    .next()
                    .context("Missing argument for `from` subcommand")?;
                let from: usize = from.parse()?;

                let to = tokens.next().context("Missing `to` subcommand")?;
                if to != "to" {
                    anyhow::bail!("Expected `to` but got {:?} instead", to);
                }

                let to = tokens
                    .next()
                    .context("Missing argument for `to` subcommand")?;
                let to: usize = to.parse()?;

                Ok(Command::Move { count, from, to })
            }
            _ => {
                anyhow::bail!("Unknown command {:?}", command);
            }
        }
    }
}

fn parse_stacks_and_commands(s: &str) -> anyhow::Result<(StackSet, Vec<Command>)> {
    let mut chunks = s.split("\n\n");

    let stacks: StackSet = chunks.next().context("Missing stacks")?.try_into()?;

    let commands: Vec<Command> = chunks
        .next()
        .context("Missing procedure")?
        .lines()
        .map(Command::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    Ok((stacks, commands))
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let (mut stacks, commands) = parse_stacks_and_commands(&input)?;

    for command in commands.iter() {
        stacks.exec(&command)?;
    }

    let message: String = stacks.top().into_iter().collect();

    dbg!(message);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let input = concat!(
            "    [D]    \n",
            "[N] [C]    \n",
            "[Z] [M] [P]\n",
            " 1   2   3 \n",
            "\n",
            "move 1 from 2 to 1\n",
            "move 3 from 1 to 3\n",
            "move 2 from 2 to 1\n",
            "move 1 from 1 to 2\n",
        );

        let (mut stacks, commands) = parse_stacks_and_commands(&input)?;

        for command in commands.iter() {
            stacks.exec(&command)?;
        }

        let message: String = stacks.top().into_iter().collect();

        assert_eq!(message, "MCD");

        Ok(())
    }
}
