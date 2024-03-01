use std::io::{self, Write};
use regex::Regex;

use crate::attitude::Attitude;

pub enum UserInput {
    Input(Attitude),
    Quit
}

use UserInput::{Input, Quit};

pub fn welcome_message() {
    println!("----------------- Welcome to the Sierra Space Assessment -----------------");
    println!("Please input the new attitude as 3 integers separated by commas or spaces");
    println!("To exit, type \"quit\" when prompted for user input\n");
}

/// Gets user input as either a message contain an Attitude or a quit message
pub fn get_user_input() -> UserInput {
    let input_validation = Regex::new(r"^\s*(-?\d+)[ ,]+(-?\d+)[ ,]+(-?\d+)\s*$").expect("Error making regex");
    loop {
        let input = prompt_user("Input values (%d %d %d): ").expect("Error getting user input");

        if input.to_lowercase() == "quit" {
            return Quit;
        }

        // returns input attitude if valid
        if let Some(att) = validate_input(&input_validation, &input) {
            return Input(att);
        }

        println!("Invalid input. Try Again.");
    }
}

pub fn goodbye_message() {
    println!("\nHave a good day!");
}

fn prompt_user(msg: &str) -> io::Result<String> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "{msg}")?;
    stdout.flush()?;

    let buf = &mut String::new();
    stdin.read_line(buf)?;

    Ok(buf.trim().to_string())
}

fn validate_input(regex: &Regex, input: &String) -> Option<Attitude> {
    let captures = regex.captures(input)?;
    let x = captures.get(1)?.as_str().parse().ok()?;
    let y = captures.get(2)?.as_str().parse().ok()?;
    let z = captures.get(3)?.as_str().parse().ok()?;
    Some(Attitude(x, y, z))
}