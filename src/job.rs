use anyhow::{Result, anyhow};
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tera::{Context, Tera};
use uuid::Uuid;

pub(crate) struct Job {
    // directory the job executes in
    working_directory: PathBuf,

    // deterministic uuid
    uuid: Uuid,

    // the time the job was created
    time: SystemTime,

    // the name of the template
    template_name: String,

    // the template context
    context: Context,

    project: String,
}

impl Job {
    pub(crate) fn new(
        working_directory: &Path,
        template_name: &str,
        context: &Vec<(String, String)>,
        project_name: &str,
    ) -> Self {
        let mut context_output = Context::new();
        let time = SystemTime::now();
        let uuid = Job::get_uuid(template_name, context, &time);
        for (key, val) in context {
            context_output.insert(key, &val);
        }
        return Job {
            working_directory: working_directory.to_path_buf(),
            uuid: uuid,
            time: SystemTime::now(),
            template_name: template_name.to_string(),
            context: context_output,
            project: project_name.to_owned(),
        };
    }

    pub(crate) fn get_uuid(
        template_name: &str,
        context: &Vec<(String, String)>,
        time: &SystemTime,
    ) -> Uuid {
        // This uuid is deterministic. We can always re-derive it when needed
        let mut data = String::from(template_name);
        for (key, val) in context {
            data.push_str(&format!("{}={}", key, val));
        }
        let dt = time.duration_since(UNIX_EPOCH).unwrap().as_secs();
        data.push_str(&format!("{}", dt));

        Uuid::new_v5(&Uuid::NAMESPACE_OID, data.as_bytes())
    }

    pub(crate) fn render_script_to_file(&self, tera: &Tera) -> Result<PathBuf> {
        let output_file_path = self.working_directory.join(format!("{}.sh", self.uuid));
        let output_file = File::create_new(&output_file_path)?;

        let write_result = tera.render_to(&self.template_name, &self.context, output_file);
        if let Err(error) = write_result {
            if let Some(source) = error.source() {
                return Err(anyhow!(format!("Error rendering job script: {}", source)));
            }

            return Err(anyhow!(format!("Error rendering job script: {}", error)));
        }

        Ok(output_file_path)
    }
}
