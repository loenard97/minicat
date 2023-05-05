use std::process;
use clap::Parser;
use minicat::{Args, Config};

fn main() {
    let args = Args::parse();
    let config = Config::new(&args);

    if let Err(e) = minicat::run(&config) {
        eprint!("Application error: {}\n", e);
        process::exit(1);
    };
}
