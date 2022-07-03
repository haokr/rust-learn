use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(value_parser)]
    file: String,
    #[clap(value_parser)]
    string: String,
}