use std::io::{BufRead, BufReader, self, Write};
use std::fs::File;

use clap::Parser;

type UniqrResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Parser)]
#[command(author = "Cristian Prieto <me@cprieto.com>", about = "Rust uniq", version = "1.0", name = "uniqr")]
pub struct Config {
    #[arg(help = "Show counts", short = 'c', long = "count")]
    count: bool,

    #[arg(help = "Input file", default_value = "-", value_name = "IN_FILE")]
    in_file: String,

    #[arg(help = "Output file", value_name = "OUT_FILE")]
    out_file: Option<String>,
}

fn open(filename: &str) -> UniqrResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(cfg: Config) -> UniqrResult<()> {
    let mut file = open(&cfg.in_file)
        .map_err(|err| format!("{}: {}", cfg.in_file, err))?;

    let mut out_file: Box<dyn Write> = match &cfg.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    let mut line = String::new();
    let mut prev = String::new();
    let mut count: u64 = 0;
    
    let mut print = |count: u64, text: &str| -> UniqrResult<()> {
        if count > 0 {
            if cfg.count {
                write!(out_file, "{:>4} {text}", count)?;
            } else {
                write!(out_file, "{text}")?;
            }
        };

        Ok(())
    };

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != prev.trim_end() {
            print(count, &prev)?;
            prev = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }

    print(count, &prev)?;
    Ok(())
}
