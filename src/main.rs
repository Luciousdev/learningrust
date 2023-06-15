use std::io;
use chrono::{Datelike, Utc, Local};

fn main() {
    let name = get_name();

    let today = Local::today();
    let day_name = today.format("%A").to_string(); 
    println!("Welcome {}\nThe day today is: {}", name, day_name);
}

fn get_name() -> String {
    println!("What is your name? ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}