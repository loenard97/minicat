use std::error::Error;
use std::fs::File;
use std::io::{self, Read, BufReader, BufRead};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Args {
    pub file_name: Option<String>,
}

pub struct Config {
    pub file_name: Option<String>,
    pub stdin_as_input: bool,
}

impl Config {
    pub fn new(args: &Args) -> Self {
        let file_name = args.file_name.clone();
        let stdin_as_input = args.file_name.is_none();

        Config { file_name, stdin_as_input }
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // output file
    if !config.stdin_as_input {
        let file = File::open(config.file_name.clone().unwrap())?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            println!("{}", line?);
        }

    // output stdin
    } else {
        let stdin = io::stdin();
        let mut buffer = String::new();
        stdin.lock().read_to_string(&mut buffer)?;
        println!("{}", buffer);
    }

    Ok(())
}
