use crate::fs::FileSystem;
use crate::parser::{Parsed1, Parsed2};
use crate::terminal::{Command, Output};
use anyhow::{anyhow, Context};
use aoc::Solver;

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn reconstruct_file_system<'a>(input: &'a Vec<Output>) -> anyhow::Result<FileSystem<'a>> {
    let mut file_system: FileSystem = Default::default();

    for (i, item) in input.iter().enumerate() {
        match item {
            Output::Command(command) => match command {
                Command::ChangeDirectory { name } => {
                    file_system
                        .cd(&name)
                        .with_context(|| format!("line {}, unable to change directory", i + 1))?;
                }
                Command::ListCurrentDirectory => {
                    file_system.ls();
                }
            },
            Output::Directory { name } => {
                file_system.see_directory(name);
            }
            Output::File { name, size } => {
                file_system.see_file(name, *size);
            }
        }
    }

    Ok(file_system)
}

#[derive(Debug, Clone)]
pub struct Solver1<'i>(pub Parsed1<'i>);

impl<'i> Solver for Solver1<'i> {
    type Solution = anyhow::Result<Solution1>;

    fn solve(&self) -> Self::Solution {
        let file_system = reconstruct_file_system(&self.0)?;

        let sizes: Vec<_> = file_system
            .directory_sizes()
            .filter(|&&size| size <= 100_000)
            .collect();

        if sizes.len() > 0 {
            Ok(sizes.into_iter().sum())
        } else {
            Err(anyhow!("no directories found"))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Solver2<'i>(pub Parsed2<'i>);

impl<'i> Solver for Solver2<'i> {
    type Solution = anyhow::Result<Solution2>;

    fn solve(&self) -> Self::Solution {
        let file_system = reconstruct_file_system(&self.0)?;

        let capacity: usize = 70_000_000;
        let available = capacity - file_system.size();

        let required: usize = 30_000_000;
        let desired = required - available;

        let sizes: Vec<_> = file_system
            .directory_sizes()
            .filter(|&&size| size >= desired)
            .collect();

        sizes
            .into_iter()
            .min()
            .cloned()
            .context("no directories found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Parser1, Parser2};
    use aoc::Parser;

    const INPUT: &'static str = include_str!("../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(Solver1(Parser1(INPUT).parse()?).solve()?, 95437);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(Solver2(Parser2(INPUT).parse()?).solve()?, 24933642);
        Ok(())
    }
}
