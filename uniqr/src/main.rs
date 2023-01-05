use clap::Parser;
use uniqr::{run, Config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::parse();
    run(cfg)
}
