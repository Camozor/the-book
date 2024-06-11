use core::panic;
use std::{
    fs::File,
    io::{self, Read},
};

#[allow(dead_code)]
pub fn execute_chapter9() {
    execute_last_char();
}

#[allow(dead_code)]
fn execute_get_username() {
    match read_username_from_file() {
        Ok(username) => println!("{}", username),
        Err(error) => panic!("{}", error),
    }
}


#[allow(dead_code)]
fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?;
    let mut username = String::from("");

    file.read_to_string(&mut username)?;

    Ok(username)
}

#[allow(dead_code)]
fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::from("");
    File::open("username.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

#[allow(dead_code)]
fn execute_last_char() {
    let c = last_char_of_first_line("hello world\nsalut");
    match c {
        Some(c) => println!("{}", c),
        None => {}
    };
}

#[allow(dead_code)]
fn last_char_of_first_line(s: &str) -> Option<char> {
    s.lines().next()?.chars().last()
}
