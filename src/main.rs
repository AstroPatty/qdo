use clap::Parser;
use std::process;

use cli::Cli;

mod cli;
mod dispatch;
mod job;
mod submit;
mod template;
mod template_store;

fn main() {
    let args = Cli::parse();
    let result = dispatch::dispatch(&args);
    if let Err(error) = result {
        eprintln!("{}", error);
        process::exit(1)
    }
}
