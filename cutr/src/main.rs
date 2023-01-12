use clap::Parser;
use cutr::{run, Opts};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    run(opts.to_config())?;
    Ok(())
}
