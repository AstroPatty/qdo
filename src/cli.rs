use anyhow::{Result, anyhow};
use clap::{ArgAction, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    Submit(SubmitArgs),
    Add(AddArgs),
}

#[derive(Parser, Debug)]
pub(crate) struct SubmitArgs {
    pub(crate) template: String,
    #[arg(short, long, value_parser=parse_key_val, action = ArgAction::Append)]
    pub(crate) context: Vec<(String, String)>,
}

#[derive(Parser, Debug)]
pub(crate) struct AddArgs {
    pub(crate) name: String,
    pub(crate) template_path: PathBuf,
    #[arg(short, help = "Overwrite an existing template")]
    pub(crate) overwrite: bool,
}

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
