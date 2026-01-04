use crate::directory::{DirType, QdoDirectory};
use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::to_vec;
use sled::{Db, Tree};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub(crate) struct Allocation {
    name: String,
    root: QdoDirectory,
}

pub(crate) fn open_allocations(db: &Db) -> Result<HashMap<String, Allocation>> {
    let mut entries = HashMap::new();
    let allocation_tree = db.open_tree("allocations")?;
    for name in allocation_tree.iter().keys() {
        if name.is_err() {
            continue;
        }
        let name = String::from_utf8(name?.to_vec())?;
        let alloc = Allocation::read_tree(&allocation_tree, &name)?;
        entries.insert(name, alloc);
    }
    return Ok(entries);
}

impl Allocation {
    pub(crate) fn new_from_path(path: PathBuf, name: String) -> Self {
        let root = QdoDirectory::new_from_path(path, DirType::Allocation);
        Allocation { name, root }
    }
    pub(crate) fn write_tree(&self, db: &Tree) -> Result<()> {
        let value = to_vec(&self.root).unwrap();

        _ = db.insert(&self.name, value)?;
        Ok(())
    }
    pub(crate) fn read_tree(db: &Tree, name: &str) -> Result<Self> {
        let entry = db.get(name)?;
        if let Some(data) = entry {
            let root: QdoDirectory = serde_json::from_str(&String::from_utf8(data.to_vec())?)?;

            return Ok(Allocation {
                name: name.to_string(),
                root,
            });
        }
        bail!("No allocation {} found", name)
    }
}
