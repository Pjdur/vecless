use std::env;
use vecless::List;

fn main() {
    let args = env::args().skip(1); // skip program name
    let list = List::new().add(args);

    println!("Command-line arguments as a list:");
    println!("{}", list);
}
