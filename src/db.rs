use crate::allocation;
use crate::allocation::{Allocation, open_allocations};
use anyhow::{Result, bail};
use dirs::config_local_dir;
use sled;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub(crate) struct QdoDb {
    inner: sled::Db,
    allocations: RefCell<HashMap<String, Allocation>>,
}

pub(crate) fn open_db() -> Result<QdoDb> {
    let user_config_path = config_local_dir().unwrap();
    let qdo_config_path = user_config_path.join("qdo");
    fs::create_dir_all(&qdo_config_path)?;
    let db = sled::open(qdo_config_path.join("qdo.db"))?;

    let allocations = open_allocations(&db)?;

    Ok(QdoDb {
        inner: db,
        allocations: RefCell::new(allocations),
    })
}

impl QdoDb {
    pub(crate) fn allocations(&self) -> Vec<String> {
        let values = self
            .allocations
            .borrow()
            .keys()
            .map(|s| s.to_owned())
            .collect();
        values
    }
    pub(crate) fn add_allocation(&self, path: PathBuf, name: &str) -> Result<()> {
        if self.allocations.borrow().contains_key(name) {
            bail!("Already have an allocation named {}", name);
        }
        let alloc = allocation::Allocation::new_from_path(path, name.to_string());
        self.allocations
            .borrow_mut()
            .insert(name.to_string(), alloc);
        Ok(())
    }
}

impl Drop for QdoDb {
    fn drop(&mut self) {
        let allocation_tree = self.inner.open_tree("allocations").unwrap();
        let allocations = self.allocations.borrow();
        for allocation in allocations.values() {
            allocation.write_tree(&allocation_tree).unwrap()
        }
    }
}
