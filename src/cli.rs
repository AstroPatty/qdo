use crate::Runnable;
use crate::add::AddArgs;
use crate::create::CreateArgs;
use crate::submit::SubmitArgs;
use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use sled::Db;
use std::rc::Rc;

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
    Create(CreateArgs),
    List,
}

impl Runnable for Command {
    fn run(&self, db: Rc<Db>) -> Result<String> {
        match self {
            Command::Submit(args) => todo!(),
            Command::Add(args) => args.run(db),
            Command::Create(args) => args.create(db),
            Command::List => Ok(String::from("list")),
        }
    }
}
