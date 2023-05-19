use std::process;

use minicat::Config;

fn main() {
    let config = Config::new();

    if let Err(e) = minicat::run(&config) {
        eprint!("Application error: {}\n", e);
        process::exit(1);
    }
}
