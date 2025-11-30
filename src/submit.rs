use crate::job::Job;
use crate::template::resolve_template;
use anyhow::{Result, anyhow};
use clap::ArgAction;
use clap::Parser;
use sled::Db;
use std::path::Path;
use std::rc::Rc;

fn parse_key_val(data: &str) -> Result<(String, String)> {
    let pos = data.find('=');
    if let Some(i) = pos {
        let (k, v) = data.split_at(i);
        Ok((k.to_owned(), v[1..].to_owned()))
    } else {
        Err(anyhow!(format!(
            "Context should be passed in format `key=value`, recieved {}",
            data
        )))
    }
}

#[derive(Parser, Debug)]
pub(crate) struct SubmitArgs {
    pub(crate) template: String,
    #[arg(short, long, value_parser=parse_key_val, action = ArgAction::Append)]
    pub(crate) context: Vec<(String, String)>,
}

impl SubmitArgs {
    pub(crate) fn run(args: &SubmitArgs, job_dir: &Path, db: Rc<Db>) -> Result<()> {
        let tera = resolve_template(&args.template, db)?;
        let job = Job::new(job_dir, &args.template, &args.context, "default");
        let result = job.render_script_to_file(&tera);
        if let Err(error) = result {
            return Err(anyhow!(error));
        }

        return Ok(());
    }
}
