use crate::cli::SubmitArgs;
use crate::job::Job;
use crate::template::resolve_template;
use anyhow::{anyhow, Result};
use std::fs::File;
use std::path::Path;
use tera::Context;

pub(crate) fn submit(args: &SubmitArgs, job_dir: &Path) -> Result<()> {
    let tera = resolve_template(&args.template)?;
    let job = Job::new(job_dir, args.template.to_str().unwrap(), &args.context);
    let result = job.render_script_to_file(&tera);
    if let Err(error) = result {
        return Err(anyhow!(error));
    }

    return Ok(());
}
