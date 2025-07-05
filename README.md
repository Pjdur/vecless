# vecless

**vecless** is a minimal, ergonomic, singly linked list implementation in Rust — no `Vec` required.

## Features

- ✅ No `Vec` or heap-allocated arrays
- ✅ Supports `.add(...)` with any iterable
- ✅ Implements `Display` for clean printing with `{}`

## Example

```rust
use vecless::List;

fn main() {
    let list = List::new().add(["a", "b", "c"]);
    println!("{}", list); // Output: [a, b, c]
}
```