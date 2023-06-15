use std::io;
use chrono::Datelike;

fn main() {
    let name = get_name();

    let daysoftheweek = [ "ma", "di","wo","do","vr","za","zo" ];

    println!("Welcome {}", name);
    for day in daysoftheweek.iter() {
        println!("The day today is: {}", day);
    }
}

fn get_name() -> String {
    println!("What is your name? ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}