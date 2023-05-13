use std::error::Error;
use std::io::{self, Read};
use atty::Stream;
use clap::Parser;

mod print;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Args {
    pub file_name: Option<String>,
    #[arg(short, long)]
    pub pretty_print: Option<bool>,
}

pub struct Config {
    pub file_name: Option<String>,
    pub pretty_print: bool,
    pub stdin_exists: bool,
}

impl Config {
    pub fn new() -> Config {
        let args = Args::parse();
        
        let file_name = args.file_name.clone();
        let pretty_print = atty::is(Stream::Stdout) && args.pretty_print.is_none() || args.pretty_print.unwrap_or_default();
        let stdin_exists = args.file_name.is_some() && atty::is(Stream::Stdin);

        Config { file_name, pretty_print, stdin_exists }
    }
}

pub fn run_on_file(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.pretty_print {
        print::header(&config.file_name.clone().unwrap());
        print::contents_pretty(&config.file_name.clone().unwrap());
        print::footer();
    } else {
        print::contents(&config.file_name.clone().unwrap());
    }

    Ok(())
}

pub fn run_on_stdin() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_to_string(&mut buffer)?;
    print!("{}", buffer);

    Ok(())
}
