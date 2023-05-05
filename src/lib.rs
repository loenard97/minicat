use std::error::Error;
use std::fs::File;
use std::io::{self, Read, BufReader, BufRead};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Args {
    pub file_name: Option<String>,
    #[arg(short, long)]
    pub pretty_print: Option<bool>,
}

pub struct Config {
    pub file_name: Option<String>,
    pub stdin_as_input: bool,
    pub pretty_print: bool,
}

impl Config {
    pub fn new(args: &Args) -> Self {
        let file_name = args.file_name.clone();
        let stdin_as_input = args.file_name.is_none();
        let pretty_print = match args.pretty_print {
            Some(val) => val,
            None => !stdin_as_input,
        };

        Config { file_name, stdin_as_input, pretty_print }
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // output file
    if !config.stdin_as_input {
        let file = File::open(config.file_name.clone().unwrap())?;
        let reader = BufReader::new(file);

        if config.pretty_print {
            println!("─────┬─────────────────────────────────────────────────────");
            println!("     │  File {}", config.file_name.clone().unwrap());
            println!("─────┼─────────────────────────────────────────────────────");
        }

        for (i, line) in reader.lines().enumerate() {
            if config.pretty_print {
                println!(" {: ^width$} │  {}", i+1, line?, width=3);
            } else {
                println!("{}", line?);
            }
        }

        if config.pretty_print {
            println!("─────┴─────────────────────────────────────────────────────");
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
