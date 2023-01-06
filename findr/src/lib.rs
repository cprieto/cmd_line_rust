use regex::Regex;
use clap::{Parser, ValueEnum};

#[derive(Debug, Eq, PartialEq, Clone, ValueEnum)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug, Parser)]
#[command(
    author = "Cristian Prieto <me@cprieto.com>",
    about = "Rust find",
    version = "1.0",
    name = "findr"
)]
pub struct Config {
    #[arg(help = "Name", value_name = "NAME", long = "name", short = 'n')]
    names: Vec<Regex>,

    #[arg(help = "Entry type", value_name = "TYPE", long = "type", short = 't')]
    entry_types: Vec<EntryType>,

    #[arg(help = "Search paths", value_name = "PATH", default_value = ".")]
    path: Vec<String>,
}

type FindrResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn run(cfg: Config) -> FindrResult<()> {
    println!("{:?}", cfg);

    Ok(())
}
