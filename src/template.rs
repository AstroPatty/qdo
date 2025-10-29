use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use tera::ErrorKind as TErrorKind;
use tera::Tera;

pub(crate) fn resolve_template(path: &Path) -> Result<Tera> {
    if !path.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("File not found: {:?}", path),
        ));
    }

    let mut tera = Tera::default();
    let result = tera.add_template_file(path, None);
    if let Err(error) = result {
        match error.kind {
            TErrorKind::Io(kind) => {
                return Err(Error::new(
                    kind,
                    format!("Unable to read template!: {}", error),
                ));
            }
            _ => {
                return Err(Error::other(format!(
                    "Unable to parse template!: {}",
                    error
                )));
            }
        }
    }
    Ok(tera)
}
