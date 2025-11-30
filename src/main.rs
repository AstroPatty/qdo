use anyhow::Result;
use clap::Parser;
use sled::Db;
use std::process;
use std::rc::Rc;

use cli::Cli;

mod add;
mod cli;
mod create;
mod db;
mod job;
mod project;
mod submit;
mod template;
mod template_store;

pub(crate) trait Runnable {
    fn run(&self, db: Rc<Db>) -> Result<String>;
}

fn handle_result<T>(result: Result<T>) -> T {
    if let Err(error) = result {
        eprintln!("{}", error);
        process::exit(1)
    }
    result.unwrap()
}

fn run(runnable: &impl Runnable, db: Rc<Db>) -> () {
    let result = runnable.run(db);
    let msg = handle_result::<String>(result);
    println!("{}", msg);
}

fn main() {
    let database = handle_result(db::get_database());
    let args = Cli::parse();
    run(&args.command, database);
}
