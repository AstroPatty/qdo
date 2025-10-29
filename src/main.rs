use clap::error::{Error, ErrorKind};
use clap::{Command, CommandFactory, Parser};
use std::process;

use cli::Cli;

mod cli;
mod dispatch;
mod submit;
mod template;
fn main() {
    let args = Cli::parse();
    let result = dispatch::dispatch(&args);
    if let Err(error) = result {
        eprintln!("{}", error);
        process::exit(1)
    }
}
