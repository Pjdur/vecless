---
title: Usage
nav_order: 3
---

# Usage Guide for `vecless`

Welcome to the `vecless` usage guide! This document provides a deeper look at how to use the crate effectively, with examples and explanations for each major feature.

---

## 📦 What Is `vecless`?

`vecless` is a minimal, singly linked list implementation in Rust that avoids using `Vec`. It’s ideal for:

- Learning how linked lists work
- Functional-style list building
- Environments where `Vec` is overkill or unavailable

---

## 🔧 Creating a List

Start with an empty list using `List::new()`:

```rust
use vecless::List;

let list: List<i32> = List::new();
assert!(list.is_empty());
```

---

## ➕ Adding Elements

### `.push(elem)`

Adds a single element to the front of the list:

```rust
let list = List::new().push(1).push(2);
assert_eq!(format!("{}", list), "[2, 1]");
```

> Note: `.push()` reverses the order of elements if used repeatedly.

---

### `.add(iterable)`

Adds multiple elements from any iterable (e.g., arrays, vectors, ranges), preserving their original order:

```rust
let list = List::new().add(["a", "b", "c"]);
assert_eq!(format!("{}", list), "[a, b, c]");
```

This method internally reverses and appends to maintain the original order of the input.

---

## 🔁 Iterating Over Elements

Use `.iter()` to get an iterator over references to the elements:

```rust
let list = List::new().add([1, 2, 3]);

for item in list.iter() {
    println!("{}", item);
}
```

You can also use iterator adapters:

```rust
let sum: i32 = list.iter().copied().sum();
assert_eq!(sum, 6);
```

---

## 🔄 Reversing the List

Use `.reverse()` to reverse the order of elements:

```rust
let list = List::new().add([1, 2, 3]);
let reversed = list.reverse();
assert_eq!(format!("{}", reversed), "[3, 2, 1]");
```

---

## 📏 Length and Emptiness

- `.len()` returns the number of elements
- `.is_empty()` checks if the list is empty

```rust
let list = List::new().add([10, 20, 30]);
assert_eq!(list.len(), 3);
assert!(!list.is_empty());
```

---

## 🖨️ Displaying the List

`vecless::List` implements `Display`, so you can print it directly:

```rust
let list = List::new().add(["x", "y"]);
println!("{}", list); // Output: [x, y]
```

---

## 🧪 Example: Chaining Methods

```rust
let list = List::new()
    .add(1..=3)
    .push(0)
    .reverse()
    .add([4, 5]);

assert_eq!(format!("{}", list), "[5, 4, 0, 1, 2, 3]");
```

---

## 🧩 Tips and Notes

- `List<T>` is fully owned and cloneable (`Clone` is implemented).
- It’s not optimized for performance — it’s designed for clarity and composability.
- If you need a doubly linked list or random access, consider `VecDeque` or `Vec`.

---

## 📚 See Also

- [API Reference](https://docs.rs/vecless)
- [Installation Guide](installation.md)
- [Crate on crates.io](https://crates.io/crates/vecless)

---

Happy hacking with `vecless`! 🦀  
If you have suggestions or want to contribute, check out the [GitHub repo](https://github.com/Pjdur/vecless).
