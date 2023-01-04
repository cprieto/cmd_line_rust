use clap::Parser;
use std::fs::File;
use std::{error::Error, io::{BufRead, BufReader, self}};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(author = "Cristian Prieto <me@cprieto.com>", about = "Rust wc", version = "1.0")]
pub struct Config {
    #[arg(num_args = 1.., value_name = "FILE", help = "Input file(s)", default_value = "-")]
    files: Vec<String>,

    #[arg(
        short = 'c',
        long = "bytes",
        help = "Show byte count",
    )]
    bytes: bool,
    #[arg(
        short = 'l',
        long = "lines",
        help = "Show line count",
    )]
    lines: bool,

    #[arg(
        short = 'm',
        long = "chars",
        help = "Show char count",
        conflicts_with = "bytes",
    )]
    chars: bool,


    #[arg(
        short = 'w',
        long = "words",
        help = "Show word count",
        default_value_ifs = [
            ("lines", "false", Some("true")),

        ],
    )]
    words: bool,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

pub fn run(cfg: Config) -> MyResult<()> {
    for filename in &cfg.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    println!("{:>8}{:>8}{:>8} {filename}", info.num_lines, info.num_words, info.num_bytes);
                }
            }
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo { num_lines, num_words, num_bytes, num_chars })
}

#[cfg(test)]
mod tests {
    use crate::{FileInfo, count};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());

        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };

        assert_eq!(info.unwrap(), expected);
    }
}
