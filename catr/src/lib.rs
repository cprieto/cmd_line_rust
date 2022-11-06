use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Opts {
    #[arg(short = 'n', long = "number", help = "Number lines")]
    number_lines: bool,

    #[arg(
        short = 'b',
        long = "number-nonblank",
        help = "Number nonblank lines",
        conflicts_with = "number_lines"
    )]
    non_blank: bool,

    #[arg(num_args = 1.., default_value = "-", value_name = "FILE", help = "Input files")]
    files: Vec<String>,
}

type CatrResult<T> = Result<T, Box<dyn std::error::Error>>;

fn open(filename: &str) -> CatrResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Opts) -> CatrResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                file.lines()
                    .filter_map(|line| line.ok())
                    .enumerate()
                    .map(|(num, line)| {
                        if config.number_lines || config.non_blank && !line.is_empty() {
                            format!("{:>6}\t{line}", num + 1)
                        } else {
                            line
                        }
                    })
                    .for_each(|line| {
                        println!("{line}");
                    });
            }
        }
    }

    Ok(())
}