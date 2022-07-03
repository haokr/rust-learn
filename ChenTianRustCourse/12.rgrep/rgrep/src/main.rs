use clap::Parser;

use crate::cli::Cli;

mod cli;
mod service;

fn main() {
    let s = service::dummy_service::Dummy_Service::default();
    let cli = Cli::new(s);
    let res = cli.run();
    println!("{:?}", res);
}
