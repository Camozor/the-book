use std::{env, process};

use minigrep::{run, Config};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });

    if let Err(e) = run(&config) {
        eprintln!("Application error {e}");
        process::exit(1);
    }
}
