use std::env;
use vecless::List;

fn main() {
    let mut args = env::args().skip(1).peekable();

    if args.peek().is_none() {
        println!("Usage: cargo run --example todo -- [--reverse] <task 1> <task 2> ...");
        return;
    }

    let mut reverse = false;
    let mut list = List::new();

    for arg in args {
        if arg == "--reverse" {
            reverse = true;
        } else {
            list = list.push(arg);
        }
    }

    if !reverse {
        list = list.reverse();
    }

    println!("📝 Your To-Do List:");
    println!("{}", list);
}
