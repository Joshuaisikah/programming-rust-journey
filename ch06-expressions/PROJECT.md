# Chapter 6 Project: Expression Evaluator

Build a mathematical expression evaluator.

## What You're Building

Parse and evaluate math expressions using Rust's expression syntax.

```bash
cargo run "2 + 3 * 4"           # 14
cargo run "if x > 10 { x * 2 } else { x }"
cargo run "let x = 5; x * x"    # 25
```

## Features

1. **Parse expressions** - Tokenize input
2. **Evaluate** - Calculate results
3. **Variables** - Store and retrieve
4. **Control flow** - if/else expressions
5. **Blocks** - Scoped expressions

## Concepts

```rust
// Everything is an expression!
let result = if condition { 10 } else { 20 };

let value = {
    let x = 5;
    x * x  // No semicolon = return value
};
```

Master Rust's expression-oriented programming! 🧮
