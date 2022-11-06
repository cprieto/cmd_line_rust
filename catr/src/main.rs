use catr::{Opts, run};
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    run(opts)?;

    Ok(())
}
