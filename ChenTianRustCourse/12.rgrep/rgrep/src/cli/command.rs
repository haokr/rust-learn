use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Command {
    #[clap(value_parser)]
    file: String,
    #[clap(value_parser)]
    targe_string: String,
}