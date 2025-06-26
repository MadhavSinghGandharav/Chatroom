
mod server;
mod client;

use std::io::{self, Write};

fn main() {
    println!("Welcome to Rust Chat!");
    println!("1. Start server");
    println!("2. Join server");

    print!("Choose: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    match choice {
        "1" => server::run(),
        "2" => client::run(),
        _ => println!("Invalid choice"),
    }
}

