use catr::{run, Opts};
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    run(opts)?;

    Ok(())
}
