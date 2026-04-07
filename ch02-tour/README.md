# Chapter 2: Mini Grep - File Structure

This project uses a modular structure with separation of concerns.

## File Structure

```
ch02-tour/
├── Cargo.toml
├── PROJECT.md          # Full implementation guide
├── README.md           # This file
└── src/
    ├── main.rs         # Entry point - parses args and calls lib
    ├── lib.rs          # Library root - exposes public API
    ├── config.rs       # Configuration and argument parsing
    ├── search.rs       # Core search logic
    ├── display.rs      # Output formatting and display
    └── error.rs        # Custom error types
```

## Module Responsibilities

### `main.rs`
- Entry point of the application
- Minimal logic - just orchestration
- Parses command-line arguments
- Calls lib functions
- Handles top-level errors and exit codes

### `lib.rs`
- Library root
- Exposes public API
- Contains `run()` function that orchestrates the workflow
- Re-exports important types from submodules

### `config.rs`
- `Config` struct to hold application settings
- Argument parsing logic
- Input validation
- Returns `Result<Config, &str>` for error handling

### `search.rs`
- Core search functionality
- `search()` function with lifetime parameters
- Returns `Vec<(usize, &str)>` with line numbers and matches
- Uses iterators for efficient searching
- Optional: case-insensitive search

### `display.rs`
- All console output logic
- `display_results()` - Shows search results
- `display_error()` - Prints errors to stderr
- `display_usage()` - Shows usage instructions
- Keeps display logic separate from business logic

### `error.rs`
- Custom `AppError` enum
- Implements `Display` and `Error` traits
- Implements `From` for automatic conversions
- Better error messages and type safety

## Implementation Order

1. **Start with `error.rs`** - Define your error types first
2. **Then `config.rs`** - Argument parsing
3. **Then `search.rs`** - Core search logic (easiest to test)
4. **Then `display.rs`** - Output formatting
5. **Wire up `lib.rs`** - Connect everything together
6. **Finally `main.rs`** - Entry point

## Benefits of This Structure

✅ **Testability** - Each module can be tested independently
✅ **Maintainability** - Clear separation of concerns
✅ **Reusability** - `lib.rs` can be used as a library by other projects
✅ **Clarity** - Each file has a single, clear purpose
✅ **Scalability** - Easy to add new features in appropriate modules

## Running the Project

```bash
# Build
cargo build

# Run
cargo run <pattern> <filename>

# Test
cargo test

# Example
cargo run "TODO" PROJECT.md
```

## Next Steps

See `PROJECT.md` for detailed implementation guide with code examples, conventions, and best practices!
