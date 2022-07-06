use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
    /// Name of the person
    #[clap(short, long, value_parser)]
    pub name: String,
}