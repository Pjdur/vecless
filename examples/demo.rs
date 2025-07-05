use vecless::List;

fn main() {
    let list = List::new().add(["rust", "is", "fun"]);
    println!("{}", list);

    let nums = List::new().add(1..=5);
    println!("{}", nums);
}