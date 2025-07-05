use std::io::{self, Write};
use vecless::List;

fn main() {
    println!("Enter words separated by spaces:");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Failed to read input: {}", e);
        return;
    }

    let words = input.trim().split_whitespace();
    let list = List::new().add(words.map(String::from));

    println!("\nYou entered:");
    println!("{}", list);
}
