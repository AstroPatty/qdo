use crate::cli::SubmitArgs;
use crate::template::resolve_template;
use std::fs::File;
use std::io::{Error, Result};
use std::path::Path;
use tera::{Context, Tera};

pub(crate) fn submit(args: &SubmitArgs, job_id: usize, job_dir: &Path) -> Result<()> {
    let tera = resolve_template(&args.template)?;
    let mut context = Context::new();
    for (key, val) in &args.context {
        context.insert(key, &val)
    }
    let script_path = job_dir.join(format!("{}.sh", job_id));

    let output = File::create_new(&script_path)?;
    let result = tera.render_to(args.template.to_str().unwrap(), &context, output);
    if let Err(error) = result {
        return Err(Error::other(format!("{}", error)));
    }

    return Ok(());
}
