use clap::Parser;
use wcr::{run, Config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Config::parse();
    run(opts)?;

    Ok(())
}
