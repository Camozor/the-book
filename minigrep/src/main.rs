use std::{env::args, process};

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = args().collect();

    let config = Config::build(&args).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });

    if let Err(e) = run(&config) {
        eprintln!("Application error {e}");
        process::exit(1);
    }
}
