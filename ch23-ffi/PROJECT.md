# Chapter 23 Project: C Library Bindings

Create Rust bindings for a C library (like SQLite).

```rust
// Call C functions from Rust safely!
let db = sqlite_open("test.db")?;
sqlite_exec(db, "SELECT * FROM users")?;
```

Master FFI! 🔗
