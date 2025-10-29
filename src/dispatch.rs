use crate::cli::Cli;
use crate::cli::Command;
use crate::submit::submit;
use std::env::current_dir;
use std::io::Result;
use std::path::Path;

pub(crate) fn dispatch(args: &Cli) -> Result<()> {
    match &args.command {
        Command::Submit(args) => {
            let path = current_dir().unwrap();
            submit(&args, 0, &path)
        }
    }
}
