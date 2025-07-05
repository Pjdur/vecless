use std::fs::File;
use std::io::{self, BufRead, BufReader};
use vecless::List;

fn main() -> io::Result<()> {
    let file = File::open("examples/input.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines().filter_map(Result::ok);
    let list = List::new().add(lines);

    println!("Contents of input.txt as a list:");
    println!("{}", list);

    Ok(())
}
