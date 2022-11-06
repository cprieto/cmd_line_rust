use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "echor",
    version = "0.1.0",
    author = "Cristian Prieto <me@cprieto.com>",
    about = "Echo tool for stuff"
)]
struct Cli {
    #[arg(short = 'n', help = "Do not print new line")]
    omit: bool,

    #[arg(value_name = "TEXT", help = "Input text", num_args = 1.., required = true)]
    text: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    let output = cli.text.join(" ");
    print!("{}{}", output, if cli.omit { "" } else { "\n" });
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn dies_without_args() -> TestResult {
        Command::cargo_bin("echor")?
            .assert()
            .failure()
            .stderr(predicate::str::contains("Usage:"));

        Ok(())
    }

    fn run(args: &[&str], expect: &'static str) -> TestResult {
        Command::cargo_bin("echor")?
            .args(args)
            .assert()
            .success()
            .stdout(expect);

        Ok(())
    }

    #[test]
    fn echos_as_is() -> TestResult {
        run(&["Hello", "World"], "Hello World\n")
    }

    #[test]
    fn echos_without_return() -> TestResult {
        run(&["-n", "Hello", "World"], "Hello World")
    }

    #[test]
    fn echos_without_return_at_the_end() -> TestResult {
        run(&["Hello", "World", "-n"], "Hello World")
    }
}
