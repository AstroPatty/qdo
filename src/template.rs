use crate::template_store::TemplateStore;
use anyhow::{Result, anyhow};

use sled::Db;
use std::rc::Rc;
use tera::ErrorKind as TErrorKind;
use tera::Tera;

pub(crate) fn resolve_template(name: &str, db: Rc<Db>) -> Result<Tera> {
    let template_store = TemplateStore::open_template_store(db)?;
    let template = template_store
        .get_template(name)?
        .ok_or(anyhow!("Couldn't find a template named {}", name))?;

    let mut tera = Tera::default();
    tera.add_raw_template(name, &template)?;
    Ok(tera)
}
