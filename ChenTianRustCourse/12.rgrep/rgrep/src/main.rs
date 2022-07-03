use clap::Parser;

mod cli;

fn main() {
    let c = cli::Args::parse();
    println!("{:?}", c);
}
