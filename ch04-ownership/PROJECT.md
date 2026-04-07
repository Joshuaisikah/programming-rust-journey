# Chapter 4 Project: Smart Text Editor Buffer

Build a text editor buffer that demonstrates ownership and move semantics.

## What You're Building

A simple text buffer with undo/redo that shows ownership in action.

```bash
cargo run
> insert "Hello World"
> undo
> redo
> save buffer.txt
```

## Features

1. **Insert/Delete text** - Ownership of String data
2. **Undo/Redo** - Move strings between stacks
3. **Copy buffer** - Clone vs Move
4. **Save to file** - Transfer ownership to file system
5. **Multiple buffers** - Demonstrate Box, Rc

## Why This Project?

- **Shows ownership clearly** - Strings move, data transfers
- **Real-world example** - Actual editor feature
- **Touches all Ch 4 concepts:**
  - Move semantics (String ownership)
  - Clone when needed
  - Box for heap allocation
  - Rc for shared buffers
  - Drop for cleanup

## Concepts Demonstrated

```rust
struct Buffer {
    content: String,           // Owned data
    history: Vec<String>,      // Owned history
    name: Box<str>,           // Heap allocated
}

// Ownership moves
let buf1 = Buffer::new();
let buf2 = buf1;  // buf1 is now invalid!

// Clone when you need both
let buf3 = buf2.clone();  // Both valid
```

Ready to master ownership! 🔑
