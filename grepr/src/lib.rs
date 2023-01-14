use anyhow::Result;
use clap::Parser;
use regex::Regex;

#[derive(Debug, Parser)]
pub struct Opts {
    #[arg(short, long, help = "Count occurences")]
    count: bool,

    #[arg(short, long, help = "Case-insensitive")]
    insensitive: bool,

    #[arg(short = 'v', long, help = "Invert match")]
    invert: bool,

    #[arg(short, long, help = "Recursive search")]
    recursive: bool,

    #[arg(value_name = "PATTERN", help = "Search pattern", required = true)]
    pattern: Regex,

    #[arg(value_name = "FILE", default_value = "-", help = "Input file(s)")]
    files: Vec<String>,
}

pub fn run(opts: Opts) -> Result<()> {
    println!("{:?}", opts);

    Ok(())
}
