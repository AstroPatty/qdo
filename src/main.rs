use sled;

mod allocation;
mod db;
mod directory;
use anyhow::Result;
use dirs::home_dir;
use std::process;

fn handle_result<T>(result: Result<T>) -> T {
    if let Err(error) = result {
        eprintln!("{}", error);
        process::exit(1)
    }
    result.unwrap()
}
fn main() {
    let qdo_db = handle_result(db::open_db());
    let root_path = home_dir().unwrap();
    handle_result(qdo_db.add_allocation(root_path, "root"));
    let allocation_names = qdo_db.allocations();
    println!("{:?}", allocation_names);
}
