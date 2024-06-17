use std::{error::Error, fs};

pub struct Config {
    search_query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Insufficient number of args");
        }
        let search_query = args[1].to_string();
        let file_path = args[2].to_string();

        Ok(Config {
            search_query,
            file_path,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    println!("{}", contents);

    Ok(())
}
