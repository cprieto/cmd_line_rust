use clap::Parser;
use anyhow::Result;
use grepr::{run, Opts};

fn main() -> Result<()> {
    let opts = Opts::parse();
    run(opts)
}
