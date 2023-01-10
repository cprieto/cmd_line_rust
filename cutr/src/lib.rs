use regex::Regex;
use clap::Parser;
use std::num::NonZeroUsize;
use std::{error::Error, ops::Range};

use clap::{arg, Arg, ArgAction, command};

type PositionList = Vec<Range<usize>>;

#[derive(Debug, Parser)]
pub struct Opts {
    #[arg(short = 'b', long = "bytes", help = "Selected bytes", value_name = "BYTES", conflicts_with_all = ["chars"])]
    bytes: bool,

    #[arg(short = 'c', long = "chars", help = "Selected chars", value_name = "CHARS", conflicts_with_all = ["bytes"])]
    chars: bool,

    #[arg(short = 'd', long = "delim", help = "Field delimiter", default_value_t = b'\t')]
    delimiter: u8,

//    #[arg(value_parser = parse_pos)]
//    fields: PositionList,
}

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
}

type CutrResult<T> = Result<T, Box<dyn Error>>;

pub fn run(opts: Opts) -> CutrResult<()> {
    println!("{:?}", &opts);
    Ok(())
}

fn parse_pos(range: &str) -> Result<PositionList, Box<dyn Error + 'static + Send + Sync>> {
    let range_re = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    range
        .split(",")
        .into_iter()
        .map(|value| {
            parse_index(value).map(|n| n..n + 1).or_else(|e| {
                range_re.captures(value).ok_or(e).and_then(|captures| {
                    let n1 = parse_index(&captures[1])?;
                    let n2 = parse_index(&captures[2])?;

                    if n1 >= n2 {
                        return Err(format!(
                            "First number in range ({}) must be lower than second number ({})",
                            n1 + 1,
                            n2 + 1
                        ));
                    }

                    Ok(n1..n2 + 1)
                })
            })
        })
        .collect::<Result<_, _>>()
        .map_err(From::from)
}

fn parse_index(idx: &str) -> Result<usize, String> {
    let value_error = || format!("illegal list value \"{idx}\"");

    idx.starts_with("+")
        .then(|| Err(value_error()))
        .unwrap_or_else(|| {
            idx.parse::<NonZeroUsize>()
                .map(|n| usize::from(n) - 1)
                .map_err(|_| value_error())
        })
}

#[cfg(test)]
mod tests {
    use super::parse_pos;

    #[test]
    fn test_parse_pos() {
        assert!(parse_pos("").is_err());
    }

    #[test]
    fn test_zero_is_error() {
        let res = parse_pos("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value \"0\"");

        let res = parse_pos("0-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value \"0\"");
    }

    #[test]
    fn test_leading_plus_is_error() {
        let res = parse_pos("+1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value \"+1\"");

        let res = parse_pos("+1-2");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value \"+1-2\"");
    }

    #[test]
    fn test_non_number_is_error() {
        let res = parse_pos("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value \"a\"");

        let res = parse_pos("1-a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value \"1-a\"");

        let res = parse_pos("a-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value \"a-1\"");
    }

    #[test]
    fn test_waky_ranges_is_error() {
        let res = parse_pos("-");
        assert!(res.is_err());

        let res = parse_pos(",");
        assert!(res.is_err());

        let res = parse_pos("1,");
        assert!(res.is_err());

        let res = parse_pos("1-");
        assert!(res.is_err());

        let res = parse_pos("1-1-1");
        assert!(res.is_err());
    }

    #[test]
    fn test_out_limits_is_error() {
        let res = parse_pos("2-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (2) must be lower than second number (1)"
        );

        let res = parse_pos("1-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (1) must be lower than second number (1)"
        );
    }

    #[test]
    fn test_acceptable_values() {
        let res = parse_pos("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("01");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("1,3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("1-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("1,7,3-5");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);

        let res = parse_pos("15,19-20");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![14..15, 18..20]);
    }
}
