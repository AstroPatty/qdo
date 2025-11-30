use crate::db::RawStore;
use anyhow::{Result, anyhow};
use sled;
use std::rc::Rc;
use std::str::from_utf8;

pub(crate) struct TemplateStore {
    store: RawStore,
}

impl TemplateStore {
    pub(crate) fn open_template_store(db: Rc<sled::Db>) -> Result<Self> {
        let store = RawStore::new(db, "templates");
        Ok(TemplateStore { store })
    }

    pub(crate) fn get_template(&self, template_name: &str) -> Result<Option<String>> {
        let data = self.store.get_tree()?.get(template_name)?;
        if let Some(d) = data {
            return from_utf8(&d)
                .map_or_else(|err| Err(anyhow!(err)), |val| Ok(Some(val.to_owned())));
        }
        Ok(None)
    }

    pub(crate) fn try_add_template(
        &mut self,
        template_name: &str,
        template: &[u8],
    ) -> Result<String> {
        // Fails if the template already exists
        if self.store.get_tree()?.contains_key(template_name)? {
            return Err(anyhow!(format!(
                "Already have a template named {}",
                template_name
            )));
        }
        self.add_template(template_name, template)
    }

    pub(crate) fn add_template(&mut self, template_name: &str, template: &[u8]) -> Result<String> {
        let _ = self
            .store
            .get_tree()?
            .insert(&template_name, template)
            .map_err(|err| anyhow!(err))?;
        Ok(format!("Sucessfully added template {}", template_name))
    }
}
