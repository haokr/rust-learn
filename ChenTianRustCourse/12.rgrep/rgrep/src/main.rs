use crate::cli::Cli;
use show::show;

mod cli;
mod service;
mod show;

fn main() {
    let s = service::SimpleService::default();
    let cli = Cli::new(s);
    let res = cli.run();
    show(&res.res);
}
