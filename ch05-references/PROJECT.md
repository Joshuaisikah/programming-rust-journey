# Chapter 5 Project: JSON Path Query Engine

Build a JSON query tool using references and lifetimes.

## What You're Building

Query JSON data without copying it - pure references!

```bash
cargo run data.json "users[0].name"
cargo run data.json "products[*].price"
```

## Features

1. **Parse JSON** - Borrow data, don't copy
2. **Query paths** - Navigate with references
3. **Filter results** - Borrow matching items
4. **No cloning** - Everything by reference!

## Why This Project?

- **All about borrowing** - Never own, always borrow
- **Lifetimes everywhere** - Function signatures with lifetimes
- **Touches all Ch 5 concepts:**
  - Immutable references (&T)
  - Mutable references (&mut T)
  - Lifetimes ('a, 'b)
  - Slices (&[T], &str)
  - Reference rules

## Concepts

```rust
// Lifetime annotations
fn query<'a>(data: &'a str, path: &str) -> Vec<&'a str> {
    // Return references into original data
}

// No copies, pure references!
let json = read_file("data.json");
let results = query(&json, "users[0]");  // Borrows!
```

Master references and lifetimes! 🔗
