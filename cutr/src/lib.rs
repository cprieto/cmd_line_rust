use clap::{ArgGroup, Parser};
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::NonZeroUsize;
use std::{error::Error, ops::Range};

type PositionList = Vec<Range<usize>>;

#[derive(Debug, Parser)]
#[command(group(ArgGroup::new("pos").required(true).args(["fields", "chars", "bytes"])))]
pub struct Opts {
    #[arg(short, long, help = "Selected fields", value_name = "FIELDS", value_parser = parse_pos)]
    fields: Option<PositionList>,

    #[arg(short, long, help = "Selected bytes", value_name = "BYTES", conflicts_with_all = ["fields", "chars"], value_parser = parse_pos)]
    bytes: Option<PositionList>,

    #[arg(short, long, help = "Selected chars", value_name = "CHARS", conflicts_with_all = ["fields", "bytes"], value_parser = parse_pos)]
    chars: Option<PositionList>,

    #[arg(short, long, help = "Field delimiter",value_parser = parse_delim, default_value = "\t")]
    delimiter: u8,

    #[arg(help = "Input file(s)", default_value = "-", value_name = "FILE")]
    files: Vec<String>,
}

impl Opts {
    pub fn to_config(self) -> Config {
        let extract = match (self.fields, self.bytes, self.chars) {
            (Some(pos), None, None) => Extract::Fields(pos),
            (None, Some(pos), None) => Extract::Bytes(pos),
            (None, None, Some(pos)) => Extract::Chars(pos),
            (None, None, None) => unreachable!("At least one of the args is required"),
            (_, _, _) => unreachable!("the arguments conflict"),
        };

        Config {
            delimiter: self.delimiter,
            extract,
            files: self.files,
        }
    }
}

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

#[derive(Debug)]
pub struct Config {
    delimiter: u8,
    extract: Extract,
    files: Vec<String>,
}

type CutrResult<T> = Result<T, Box<dyn Error>>;

pub fn run(cfg: Config) -> CutrResult<()> {
    for filename in &cfg.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(reader) => match &cfg.extract {
                Extract::Chars(pos) => reader
                    .lines()
                    .filter_map(|line| line.ok())
                    .for_each(|line| println!("{}", extract_chars(&line, pos))),

                Extract::Bytes(pos) => reader
                    .lines()
                    .filter_map(|line| line.ok())
                    .for_each(|line| println!("{}", extract_bytes(&line, pos))),
                    
                Extract::Fields(pos) => {
                    let mut reader = ReaderBuilder::new()
                        .delimiter(cfg.delimiter)
                        .has_headers(false)
                        .from_reader(reader);

                    let mut writer = WriterBuilder::new()
                        .delimiter(cfg.delimiter)
                        .from_writer(io::stdout());

                    for record in reader.records() {
                        let record = record?;
                        writer.write_record(extract_fields(&record, pos))?;
                    }
                }
            },
        }
    }
    Ok(())
}

fn extract_chars(line: &str, char_pos: &[Range<usize>]) -> String {
    let chars: Vec<_> = line.chars().collect();
    let mut output: Vec<char> = vec![];

    for range in char_pos.into_iter().cloned() {
        for i in range {
            if let Some(val) = chars.get(i) {
                output.push(*val);
            }
        }
    }

    output.iter().collect()
}

fn extract_bytes(line: &str, byte_pos: &[Range<usize>]) -> String {
    let bytes = line.as_bytes();
    let output: Vec<_> = byte_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| bytes.get(i).copied()))
        .collect();

    String::from_utf8_lossy(&output).to_string()
}

fn extract_fields<'rec>(record: &'rec StringRecord, field_pos: &[Range<usize>]) -> Vec<&'rec str> {
    field_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| record.get(i)))
        .collect()
}

fn open(filename: &str) -> CutrResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn parse_delim(delim: &str) -> Result<u8, Box<dyn Error + Send + Sync + 'static>> {
    let bytes = delim.as_bytes();
    if bytes.len() != 1 {
        return Err(From::from(format!(
            "--delim \"{}\" must be a single byte",
            delim
        )));
    }

    Ok(bytes[0])
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
