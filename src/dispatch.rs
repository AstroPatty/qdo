use crate::add::add_template;
use crate::cli::Cli;
use crate::cli::Command;
use crate::submit::submit;
use anyhow::Result;
use std::env::current_dir;

pub(crate) fn dispatch(args: &Cli) -> Result<()> {
    match &args.command {
        Command::Submit(args) => {
            let path = current_dir().unwrap();
            submit(&args, &path)
        }
        Command::Add(args) => add_template(&args),
    }
}
