use anyhow::{Result, anyhow};
use dirs::data_dir;
use sled;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::str::from_utf8;

pub(crate) struct TemplateStore {
    db: sled::Db,
}

impl TemplateStore {
    fn get_store_path() -> Result<PathBuf> {
        let data_dir = data_dir().ok_or(anyhow!("Unable to locate user data dir"))?;
        let qdo_dir = data_dir.join("qdo");
        create_dir_all(&qdo_dir)?;

        Ok(qdo_dir.join("templates.db"))
    }

    pub(crate) fn open_template_store() -> Result<Self> {
        let db_path = TemplateStore::get_store_path()?;
        let db = sled::open(db_path).map_err(|err| anyhow!(err))?;
        Ok(TemplateStore { db })
    }

    pub(crate) fn get_template(&self, template_name: &str) -> Result<Option<String>> {
        let data = self.db.get(template_name)?;
        if let Some(d) = data {
            return from_utf8(&d)
                .map_or_else(|err| Err(anyhow!(err)), |val| Ok(Some(val.to_owned())));
        }
        Ok(None)
    }

    pub(crate) fn try_add_template(&mut self, template_name: &str, template: &[u8]) -> Result<()> {
        // Fails if the template already exists
        if self.db.contains_key(template_name)? {
            return Err(anyhow!(format!(
                "Already have a template named {}",
                template_name
            )));
        }
        self.add_template(template_name, template)
    }

    pub(crate) fn add_template(&mut self, template_name: &str, template: &[u8]) -> Result<()> {
        let _ = self
            .db
            .insert(&template_name, template)
            .map_err(|err| anyhow!(err))?;
        Ok(())
    }
}
