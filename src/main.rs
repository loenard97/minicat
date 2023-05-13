use std::process;

use minicat::Config;

fn main() {
    let config = Config::new();

    if let Err(e) = if config.stdin_exists {
        minicat::run_on_file(&config)
    } else {
        minicat::run_on_stdin()
    } {
        eprint!("Application error: {}\n", e);
        process::exit(1);
    }
}
