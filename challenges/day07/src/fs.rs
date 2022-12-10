use anyhow::Context;
use std::collections::{HashMap, HashSet};

type ID = usize;

#[derive(Debug, Default)]
pub struct FileSystem<'a> {
    ids: ID,
    current_directory: ID,
    files: HashSet<ID>,
    directories: HashSet<ID>,
    sizes: HashMap<ID, usize>,
    parents: HashMap<ID, ID>,
    children: HashMap<ID, HashMap<&'a str, ID>>,
}

impl<'a> FileSystem<'a> {
    fn new_id(&mut self) -> ID {
        self.ids += 1;
        self.ids
    }

    pub fn cd(&mut self, name: &'a str) -> anyhow::Result<()> {
        match name {
            "/" => self.current_directory = 0,
            ".." => {
                if self.current_directory != 0 {
                    self.current_directory =
                        *self.parents.get(&self.current_directory).with_context(|| {
                            format!("no parent directory for {:?}", self.current_directory)
                        })?;
                }
            }
            _ => {
                let &child_id = self
                    .children
                    .entry(self.current_directory)
                    .or_default()
                    .get(name)
                    .with_context(|| format!("no such directory: {:?}", name))?;

                if self.files.contains(&child_id) {
                    anyhow::bail!("not a directory: {:?}", name);
                }

                self.current_directory = child_id;
            }
        }

        Ok(())
    }

    pub fn ls(&mut self) {
        let size = *self.sizes.entry(self.current_directory).or_default();

        let mut id = self.current_directory;

        loop {
            self.sizes.entry(id).and_modify(|value| *value -= size);
            if let Some(&parent_id) = self.parents.get(&id) {
                id = parent_id;
            } else {
                break;
            }
        }

        for (_, child_id) in self
            .children
            .entry(self.current_directory)
            .or_default()
            .drain()
        {
            self.sizes.remove(&child_id);
            self.parents.remove(&child_id);
            self.files.remove(&child_id);
            self.directories.remove(&child_id);
        }
    }

    fn see_path(&mut self, name: &'a str) -> ID {
        let id = self.new_id();

        self.parents.insert(id, self.current_directory);

        self.children
            .entry(self.current_directory)
            .or_default()
            .insert(name, id);

        id
    }

    pub fn see_directory(&mut self, name: &'a str) {
        let id = self.see_path(name);
        self.directories.insert(id);
    }

    pub fn see_file(&mut self, name: &'a str, size: usize) {
        let mut id = self.see_path(name);

        self.files.insert(id);
        self.sizes.insert(id, size);

        while let Some(parent_id) = self.parents.get(&id) {
            id = *parent_id;
            *self.sizes.entry(id).or_default() += size;
        }
    }

    pub fn size(&self) -> usize {
        self.sizes[&0]
    }

    pub fn directory_sizes(&self) -> impl Iterator<Item = &usize> {
        self.directories
            .iter()
            .map(|id| self.sizes.get(id).unwrap())
    }
}
