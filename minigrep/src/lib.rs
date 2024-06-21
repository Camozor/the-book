use std::{env, error::Error, fs};

pub struct Config {
    search_query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Insufficient number of args");
        }
        let search_query = args[1].to_string();
        let file_path = args[2].to_string();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            search_query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let found_lines = if config.ignore_case {
        search_insensitive(&config.search_query, &contents)
    } else {
        search(&config.search_query, &contents)
    };
    for found_line in found_lines {
        println!("{}", found_line);
    }

    Ok(())
}

pub fn search<'a>(search_query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .into_iter()
        .filter(|line| line.contains(search_query))
        .collect()
}

pub fn search_insensitive<'a>(search_query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .into_iter()
        .filter(|line| line.to_uppercase().contains(&search_query.to_uppercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let search_query = "Hello";
        let contents = "\
Bonjour, ça va ?
Hello, how are you?
Hola, ¿cómo está?";
        assert_eq!(search(search_query, contents), vec!["Hello, how are you?"]);
    }

    #[test]
    fn case_insensitive() {
        let search_query = "HELLO";
        let contents = "\
Bonjour, ça va ?
Hello, how are you?
Hola, ¿cómo está?";
        assert_eq!(
            search_insensitive(search_query, contents),
            vec!["Hello, how are you?"]
        );
    }
}
