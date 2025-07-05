use vecless::List;

fn main() {
    let list = List::new().add(["a", "b"]);
    println!("Length: {}", list.len());       // → 2
    println!("Is empty? {}", list.is_empty()); // → false
}