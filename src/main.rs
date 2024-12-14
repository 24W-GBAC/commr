use crate::Column::*;
use anyhow::{anyhow, bail, Result};
use clap::{ArgAction, Parser};
use std::{
    cmp::Ordering::*,
    cmp::max,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg()]
    file1: String,

    #[arg()]
    file2: String,

    /// Suppress printing of column 1
    #[arg(short('1'), action(ArgAction::SetFalse))]
    show_col1: bool,

    /// Suppress printing of column 2
    #[arg(short('2'), action(ArgAction::SetFalse))]
    show_col2: bool,

    /// Suppress printing of column 3
    #[arg(short('3'), action(ArgAction::SetFalse))]
    show_col3: bool,

    #[arg(short)]
    insensitive: bool,

    #[arg(short, long("output-delimiter"), default_value = "\t")]
    delimiter: String,
}


enum Column<'a> {
    Col1(&'a str),
    Col2(&'a str),
    Col3(&'a str),
}

// --------------------------------------------------
fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

// --------------------------------------------------
fn run(args: Args) -> Result<()> {
    let filename1 = &args.file1;
    let filename2 = &args.file2;

    if filename1 == "-" && filename2 == "-" {
        bail!(r#"Both input files cannot be STDIN ("-")"#);
    }

    // Collect lines into Vec<Result<String, io::Error>>
    let first: Vec<String> = open(&filename1)?.lines().collect::<Result<_, _>>()?;
    let second: Vec<String> = open(&filename2)?.lines().collect::<Result<_, _>>()?;

    let mut both: Vec<String> = Vec::new();

    // Find lines that are present in both files
    for line1 in &first {
        for line2 in &second {
            if line1 == line2 {
                both.push(line1.clone());
            }
        }
    }

        // Filter lines equal to the content in both in `first` and `second`
        let mut first_filtered: Vec<&String> = first.iter().filter(|&line| !both.contains(line)).collect();
        let mut second_filtered: Vec<&String> = second.iter().filter(|&line| !both.contains(line)).collect();

    // Print first_filtered, second_filtered, and both in three columns
    let binding = vec![first_filtered.len(), second_filtered.len(), both.len()];
    let maxlen = binding.iter().max().expect("");
    let empty = "".to_string();
    first_filtered.resize(*maxlen, &empty);
    second_filtered.resize(*maxlen, &empty);
    both.resize(*maxlen, "".to_string());

    for i in 0..*maxlen {
        println!("{}\t{}\t{}", first_filtered[i], second_filtered[i], both[i]);
    }

    Ok(())
}


// --------------------------------------------------
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename).map_err(|e| anyhow!("{filename}: {e}"))?,
        ))),
    }
}
