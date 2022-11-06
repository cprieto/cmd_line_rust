use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author = "Cristian Prieto <me@cprieto.com>", about = "Rust cat clone")]
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

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use std::fs;

    type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn it_can_output_two_files() -> TestResult<()> {
        let mut expected = fs::read_to_string("Cargo.lock")?;
        expected.push_str(&fs::read_to_string("Cargo.toml")?);

        Command::cargo_bin("catr")?
            .args(&["Cargo.lock", "Cargo.toml"])
            .assert()
            .success()
            .stdout(expected);

        Ok(())
    }

    #[test]
    fn it_can_output_stdin() -> TestResult<()> {
        Command::cargo_bin("catr")?
            .write_stdin("hello")
            .assert()
            .success()
            .stdout("hello\n");

        Ok(())
    }

    #[test]
    fn it_can_output_num_lines() -> TestResult<()> {
        Command::cargo_bin("catr")?
            .arg("-n")
            .write_stdin("hello\nworld")
            .assert()
            .success()
            .stdout("     1\thello\n     2\tworld\n");

        Ok(())
    }

    #[test]
    fn it_can_omit_num_empty_lines() -> TestResult<()> {
        Command::cargo_bin("catr")?
            .arg("-b")
            .write_stdin("hello\n\nworld")
            .assert()
            .success()
            .stdout("     1\thello\n\n     3\tworld\n");

        Ok(())
    }

    #[test]
    fn b_and_n_cannot_be_used_together() -> TestResult<()> {
        Command::cargo_bin("catr")?
            .args(&["-b", "-n"])
            .assert()
            .failure();

        Ok(())
    }
}
