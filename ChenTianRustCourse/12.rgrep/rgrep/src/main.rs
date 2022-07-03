use clap::Parser;

use crate::cli::Command;

mod cli;

fn main() {
    let c = Command::parse();
    println!("{:?}", c);
}
