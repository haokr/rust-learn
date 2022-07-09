use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Command {
    #[clap(value_parser)]
    pub file: String,
    #[clap(value_parser)]
    pub target_string: String,
}