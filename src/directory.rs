use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum DirType {
    Allocation,
    Project,
    Resource,
    Artifact,
}

/*
Qdo treats all directories as equivalent, regardless of semantic meaning. Directories can contain other directories.
*/

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct QdoDirectory {
    location: PathBuf,
    pub(crate) dirtype: DirType,
    children: RefCell<HashMap<String, QdoDirectory>>,
}

impl QdoDirectory {
    pub(crate) fn new_from_path(path: PathBuf, type_: DirType) -> Self {
        QdoDirectory {
            location: path,
            dirtype: type_,
            children: RefCell::new(HashMap::new()),
        }
    }

    fn get_path(&self) -> &Path {
        &self.location
    }
    fn add_child(&mut self, name: String, child: QdoDirectory, overwrite: bool) -> Result<()> {
        if !overwrite && self.children.borrow().contains_key(&name) {
            bail!("This directory already has a child named {}", name)
        }
        self.children.borrow_mut().insert(name, child);
        Ok(())
    }

    fn add_children(&mut self, mut new_children: HashMap<String, QdoDirectory>) -> Result<()> {
        let has_overlaps = {
            let children = self.children.borrow();
            children.keys().any(|cn| new_children.contains_key(cn))
        };
        if has_overlaps {
            bail!("Child names must be unique!")
        }

        let mut self_children = self.children.borrow_mut();
        for (key, value) in new_children.drain() {
            self_children.insert(key, value);
        }
        Ok(())
    }
}
