use anyhow::{Result, anyhow};
use std::path::Path;
use tera::ErrorKind as TErrorKind;
use tera::Tera;

pub(crate) fn resolve_template(path: &Path) -> Result<Tera> {
    if !path.exists() {
        return Err(anyhow!(format!("File not found: {:?}", path)));
    }

    let mut tera = Tera::default();
    let result = tera.add_template_file(path, None);
    if let Err(error) = result {
        match error.kind {
            TErrorKind::Io(_) => {
                return Err(anyhow!(format!("Unable to read template!: {}", error)));
            }
            _ => {
                return Err(anyhow!(format!("Unable to parse template!: {}", error)));
            }
        }
    }
    Ok(tera)
}
