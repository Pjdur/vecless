---
title: Tutorial
nav_order: 4
---

# Building a To-Do App with `vecless`

In this tutorial, you'll build a command-line To-Do list app using the `vecless` crate. This app will:

- Accept tasks as command-line arguments
- Support a `--reverse` flag to flip the order
- Support a `--filter <keyword>` flag to show only matching tasks
- Display tasks with line numbers

---

## 🧱 Step 1: Set Up Your Project

Create a new binary project:

```bash
cargo new vecless-todo --bin
cd vecless-todo
```

Add `vecless` to your `Cargo.toml`:

```toml
[dependencies]
vecless = "0.1"
```

---

## ✍️ Step 2: Write the App

Replace the contents of `src/main.rs` with the following:

```rust
use std::env;
use vecless::List;

fn main() {
    let mut args = env::args().skip(1).peekable();

    if args.peek().is_none() {
        println!("Usage: cargo run -- [--reverse] [--filter <keyword>] <task 1> <task 2> ...");
        return;
    }

    let mut reverse = false;
    let mut filter: Option<String> = None;
    let mut list = List::new();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--reverse" => reverse = true,
            "--filter" => {
                if let Some(keyword) = args.next() {
                    filter = Some(keyword);
                } else {
                    eprintln!("Expected keyword after --filter");
                    return;
                }
            }
            task => list = list.push(task.to_string()),
        }
    }

    if !reverse {
        list = list.reverse();
    }

    if let Some(keyword) = filter {
        list = list
            .iter()
            .filter(|task| task.contains(&keyword))
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .fold(List::new(), |list, task| list.add([task]));
    }

    if list.is_empty() {
        println!("✅ No tasks to show.");
    } else {
        println!("📝 Your To-Do List:");
        for (i, task) in list.iter().enumerate() {
            println!("{}. {}", i + 1, task);
        }
    }
}
```

---

## 🧪 Step 3: Try It Out

### Basic usage:

```bash
cargo run -- "Buy milk" "Read Rust book" "Walk the dog"
```

### Reverse order:

```bash
cargo run -- --reverse "Buy milk" "Read Rust book" "Walk the dog"
```

### Filter tasks:

```bash
cargo run -- --filter Rust "Buy milk" "Read Rust book" "Walk the dog"
```

---

## 🧠 How It Works

- `List::new()` creates an empty list.
- `.push(...)` adds tasks to the front (so we reverse at the end to preserve order).
- `.reverse()` flips the list if `--reverse` is not passed.
- `.iter()` gives us a reference-based iterator for filtering and printing.
- We use `.cloned()` and `.collect()` to rebuild a filtered list.

---

## 🧩 Bonus Ideas

- Add a `--numbered` flag to toggle line numbers
- Save tasks to a file and load them later
- Add task completion and removal
