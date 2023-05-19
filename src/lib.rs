use std::error::Error;
use std::fs::File;
use std::io::{Read, stdin};
use atty::Stream;
use clap::Parser;
use colorama::Colored;

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
    pub stdout_exists: bool,
}

impl Config {
    pub fn new() -> Config {
        let args = Args::parse();
        
        let file_name = args.file_name.clone();
        let pretty_print = atty::is(Stream::Stdout) && args.pretty_print.is_none() || args.pretty_print.unwrap_or_default();
        let stdin_exists = args.file_name.is_some() && atty::is(Stream::Stdin);
        let stdout_exists = atty::is(Stream::Stdout);

        Config { file_name, pretty_print, stdin_exists, stdout_exists }
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file_name = match &config.file_name {
        Some(val) => &val,
        None => "<stdin>",
    };
    let mut buffer = String::new();

    if config.stdin_exists {
        let mut file = File::open(file_name.clone()).unwrap();
        file.read_to_string(&mut buffer)?;
    } else {
        let mut file = stdin().lock();
        file.read_to_string(&mut buffer)?;
    }

    if config.pretty_print {
        println!("{}", header(&file_name));
    }

    for (i, line) in buffer.lines().enumerate() {
        println!("{}", apply_pretty_print(config, line, i));
    }

    if config.pretty_print {
        println!("{}", footer());
    }

    Ok(())
}

fn header(file_name: &str) -> String {
    let mut result = String::new();
    let n_cols: usize = termsize::get().unwrap().cols.into();

    result.push_str(&format!("─────┬{}\n", "─".repeat(n_cols - 6)));
    result.push_str(&format!("     │  File {}\n", file_name.to_string().style("bold")));
    result.push_str(&format!("─────┼{}", "─".repeat(n_cols - 6)));

    result
}

fn apply_pretty_print(config: &Config, content: &str, line_number: usize) -> String {
    let mut result = String::new();

    if config.pretty_print {
        result = format!(" {: ^width$} │  ", line_number + 1, width=3);
    }
    result.push_str(content.trim_end_matches(char::is_whitespace));

    result
}

fn footer() -> String {
    let mut result = String::new();
    let n_cols: usize = termsize::get().unwrap().cols.into();

    result.push_str(&format!("─────┴{}", "─".repeat(n_cols - 6)));

    result
}
