use anyhow::{Result, anyhow};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tera::{Context, Tera};
use uuid::Uuid;

pub(crate) struct Job {
    // directory the job executes in
    working_directory: PathBuf,

    // the time the job was created
    time: SystemTime,

    // the name of the template
    template_name: String,

    // the template context
    context: Vec<(String, String)>,
}

impl Job {
    fn new(working_directory: &Path, template_name: &str, context: Vec<(String, String)>) -> Self {
        return Job {
            working_directory: working_directory.to_path_buf(),
            time: SystemTime::now(),
            template_name: template_name.to_string(),
            context,
        };
    }

    pub(crate) fn get_uuid(&self) -> Uuid {
        let mut data = String::from(&self.template_name);
        for (key, val) in &self.context {
            data.push_str(&format!("{}={}", key, val));
        }
        let dt = self.time.duration_since(UNIX_EPOCH).unwrap().as_secs();
        data.push_str(&format!("{}", dt));

        Uuid::new_v5(&Uuid::NAMESPACE_OID, data.as_bytes())
    }

    fn generate_context(&self) -> Context {
        let mut context = Context::new();
        for (key, val) in &self.context {
            context.insert(key, &val);
        }
        context
    }

    pub(crate) fn render_script_to_file(&self, tera: &Tera) -> Result<PathBuf> {
        let uuid = self.get_uuid();
        let output_file_path = self.working_directory.join(format!("{}.sh", uuid));
        let output_file = File::create_new(&output_file_path)?;

        let context = self.generate_context();
        let write_result = tera.render_to(&self.template_name, &context, output_file);
        if let Err(error) = write_result {
            return Err(anyhow!(format!("Error rendering job script: {}", error)));
        }

        Ok(output_file_path)
    }
}
