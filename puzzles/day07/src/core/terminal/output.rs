use std::convert::TryFrom;

use anyhow::{anyhow, Context};

use crate::core::FileSystem;

use super::Command;

#[derive(Debug, Clone)]
pub enum Output<'a> {
    Command(Command<'a>),
    Directory { name: &'a str },
    File { name: &'a str, size: usize },
}

impl<'a> TryFrom<&'a str> for Output<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut tokens = s.split_whitespace();
        let first = tokens.next().unwrap();

        if first == "$" {
            let command = tokens.next().context("missing command")?;
            if command == "cd" {
                let name = tokens.next().context("missing directory for cd command")?;
                Ok(Self::Command(Command::ChangeDirectory { name }))
            } else if command == "ls" {
                Ok(Self::Command(Command::ListCurrentDirectory))
            } else {
                Err(anyhow!("unrecognized command: {:?}", command))
            }
        } else if first == "dir" {
            let name = tokens.next().context("missing directory")?;
            Ok(Self::Directory { name })
        } else {
            let size: usize = first
                .parse()
                .with_context(|| format!("invalid integer: {:?}", first))?;
            let name = tokens.next().context("missing file name")?;
            Ok(Self::File { name, size })
        }
    }
}

#[derive(Debug, Clone)]
pub struct OutputLines<'a>(pub Vec<Output<'a>>);

impl<'a> OutputLines<'a> {
    pub fn reconstruct_file_system(&self) -> anyhow::Result<FileSystem<'a>> {
        let mut file_system: FileSystem = Default::default();

        for (i, item) in self.0.iter().enumerate() {
            match item {
                Output::Command(command) => match command {
                    Command::ChangeDirectory { name } => {
                        file_system.set_directory(&name).with_context(|| {
                            format!("line {}, unable to change directory", i + 1)
                        })?;
                    }
                    Command::ListCurrentDirectory => {
                        file_system.clear_paths();
                    }
                },
                Output::Directory { name } => {
                    file_system.add_directory(name);
                }
                Output::File { name, size } => {
                    file_system.add_file(name, *size);
                }
            }
        }

        Ok(file_system)
    }
}

impl<'a> TryFrom<&'a str> for OutputLines<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self(
            s.lines()
                .enumerate()
                .filter(|(_, s)| s.trim().len() > 0)
                .map(|(i, s)| {
                    s.try_into()
                        .with_context(|| format!("line number {}", i + 1))
                })
                .collect::<Result<Vec<_>, _>>()
                .context("unable to parse terminal output")?,
        ))
    }
}
