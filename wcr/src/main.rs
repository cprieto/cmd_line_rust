use clap::Parser;
use wcr::{run, Config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Config::parse();
    run(opts)?;

    Ok(())
}

#[cfg(test)]
mod test {

    use assert_cmd::Command;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn run(args: &[&str], expect: &'static str) -> TestResult {
        Command::cargo_bin("wcr")?
            .args(args)
            .assert()
            .success()
            .stdout(predicates::str::starts_with(expect));

        Ok(())
    }

    #[test]
    fn test_no_params() -> TestResult {
        run(&["tests/inputs/fox.txt"], "       1       9      48 tests/inputs/fox.txt")
    }

    #[test]
    fn test_byte_and_chars_fail() -> TestResult {
        Command::cargo_bin("wcr")?
            .args(&["-m", "-c"])
            .assert()
            .failure()
            .stderr(predicates::str::contains("The argument '--chars' cannot be used with '--bytes'"));

        Ok(())
    }

    #[test]
    fn test_empty_file() -> TestResult {
        run(&["tests/inputs/empty.txt"],
            "       0       0       0 tests/inputs/empty.txt"
        )
    }

    #[test]
    fn test_handles_unicode() -> TestResult {
        run(&["tests/inputs/atlamal.txt"],"       4      29     177 tests/inputs/atlamal.txt")
    }
}
