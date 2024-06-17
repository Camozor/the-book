use std::{env::args, process::exit};

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = args().collect();

    let config = Config::build(&args).unwrap_or_else(|error| {
        println!("{}", error);
        exit(1);
    });

    if let Err(e) = run(&config) {
        println!("Application error {e}");
        exit(1);
    }
}


