use anyhow::{anyhow, Context};

#[derive(Debug, Clone)]
pub enum Command<'a> {
    ListCurrentDirectory,
    ChangeDirectory { name: &'a str },
}

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
