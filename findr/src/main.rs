use clap::Parser;
use findr::{run, Config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::parse();

    run(cfg)
}
