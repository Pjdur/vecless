# vecless

**vecless** is a minimal, ergonomic, singly linked list implementation in Rust — no `Vec` required.

## ✨ Features

- ✅ No `Vec` or heap-allocated arrays
- ✅ Supports `.add(...)` with any iterable
- ✅ Implements `Display` for clean, human-readable output
- ✅ Iterator support for `for` loops and `.iter()`

## 🚀 Example

```rust
use vecless::List;

fn main() {
    let list = List::new().add(["a", "b", "c"]);
    println!("{}", list); // Output: [a, b, c]
}
```

## Why Vecless?

Rust’s built-in `Vec<T>` is powerful and flexible — but it doesn’t implement the `Display` trait, which means you can’t print it with `{}`. Instead, you have to use the debug formatter `{:?}`:

```rust
let v = vec![1, 2, 3];
println!("{:?}", v); // [1, 2, 3]
```

`vecless` was created to:

- Provide a list type that implements `Display` out of the box
- Offer a lightweight, composable alternative to `Vec` for learning and experimentation
- Help developers understand how linked lists work under the hood in Rust
- Serve as a teaching tool or a minimal data structure for constrained environments

While not a replacement for `Vec` in performance-critical code, `vecless` is great for:
- Educational projects
- Functional-style list building
- CLI tools or embedded contexts where simplicity matters

---

Want to contribute or suggest improvements? [Open an issue](https://github.com/Pjdur/vecless/issues) or start a discussion!
