---
title: Installation
nav_order: 2
---

# 📦 Installation Guide for `vecless`

Getting started with `vecless` is quick and easy. Follow the steps below to add it to your Rust project.

---

## 🛠️ Step 1: Add `vecless` to Your `Cargo.toml`

Open your project’s `Cargo.toml` file and add the following line under `[dependencies]`:

```toml
vecless = "0.1"
```

> 💡 Replace `"0.1"` with the latest version if a newer one is available:  
> [Check crates.io →](https://crates.io/crates/vecless)

---

## 🧪 Step 2: Use It in Your Code

Import the crate and start building lists:

```rust
use vecless::List;

fn main() {
    let list = List::new().add(["a", "b", "c"]);
    println!("{}", list); // Output: [a, b, c]
}
```

---

## ✅ Step 3: Build and Run

Compile and run your project to verify everything works:

```bash
cargo build
cargo run
```

You should see your list printed to the console.

---

## 🔍 Optional: Enable Docs Locally

To explore the documentation offline:

```bash
cargo doc --open
```

This will generate and open the full API documentation in your browser.

---

## 🧩 Troubleshooting

- **Crate not found?** Make sure you're using the correct crate name: `vecless`
- **Outdated version?** Run `cargo update` to refresh your dependencies
- **Still stuck?** [Open an issue](https://github.com/Pjdur/vecless/issues) on GitHub
