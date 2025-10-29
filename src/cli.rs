use clap::{ArgAction, Parser, Subcommand};
use std::io::{Error, Result};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Submit(SubmitArgs),
}

#[derive(Parser, Debug)]
struct SubmitArgs {
    template: String,
    #[arg(short, long, value_parser=parse_key_val, action = ArgAction::Append)]
    context: Vec<(String, String)>,
}

fn parse_key_val(data: &str) -> Result<(String, String)> {
    let pos = data.find('=');
    if let Some(i) = pos {
        let (k, v) = data.split_at(i);
        Ok((k.to_owned(), v[1..].to_owned()))
    } else {
        Err(Error::other(format!("Expected a key-value pair")))
    }
}
