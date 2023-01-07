use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};
use regex::Regex;
use walkdir::WalkDir;

#[derive(Debug, Eq, PartialEq, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Dir => Some(PossibleValue::new("d")),
            Self::File => Some(PossibleValue::new("f")),
            Self::Link => Some(PossibleValue::new("l")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Dir, Self::File, Self::Link]
    }
}

#[derive(Debug, Parser)]
#[command(
    author = "Cristian Prieto <me@cprieto.com>",
    about = "Rust find",
    version = "1.0",
    name = "findr"
)]
pub struct Config {
    #[arg(help = "Name", value_name = "NAME", long = "name", short = 'n')]
    names: Vec<Regex>,

    #[arg(help = "Entry type", value_name = "TYPE", long = "type", short = 't')]
    entry_types: Vec<EntryType>,

    #[arg(help = "Search paths", value_name = "PATH", default_value = ".")]
    paths: Vec<String>,
}

type FindrResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn run(cfg: Config) -> FindrResult<()> {
    for path in cfg.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(err) => eprint!("{}", err),
                Ok(entry) => {
                    if (cfg.entry_types.is_empty()
                        || cfg.entry_types.iter().any(|et| match et {
                            EntryType::Dir => entry.file_type().is_dir(),
                            EntryType::File => entry.file_type().is_file(),
                            EntryType::Link => entry.file_type().is_symlink(),
                        }))
                        && (cfg.names.is_empty()
                            || cfg
                                .names
                                .iter()
                                .any(|re| re.is_match(&entry.file_name().to_string_lossy())))
                    {
                        println!("{}", entry.path().display());
                    }
                }
            }
        }
    }
    Ok(())
}
