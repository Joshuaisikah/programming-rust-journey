# Chapter 2 Project: Mini Grep - File Search Tool

Build a command-line tool that searches for text patterns in files.

---

## 🎯 Learning Goals

- Parse command-line arguments
- Read files efficiently
- Search strings with pattern matching
- Handle errors gracefully
- Write tests
- Use Vec and iterators

---

## 📋 Project Structure

```rust
// Recommended file organization:

use std::env;          // For command-line args
use std::fs;           // For file operations
use std::process;      // For exit codes

fn main() {
    // 1. Parse arguments
    // 2. Call run() function
    // 3. Handle errors
}

fn run(pattern: &str, filename: &str) -> Result<(), String> {
    // Main logic here
}

fn search<'a>(pattern: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    // Search logic returns (line_number, line_text) tuples
}

#[cfg(test)]
mod tests {
    // Unit tests here
}
```

---

## 🔨 Implementation Guide

### Step 1: Parse Command-Line Arguments

**Convention:** Use `std::env::args()` to get arguments

```rust
fn main() {
    // Collect arguments into a Vec<String>
    let args: Vec<String> = env::args().collect();

    // Check if we have enough arguments
    // args[0] is the program name
    // args[1] should be the pattern
    // args[2] should be the filename
    if args.len() < 3 {
        eprintln!("Usage: {} <pattern> <filename>", args[0]);
        eprintln!("Example: {} \"TODO\" src/main.rs", args[0]);
        process::exit(1);  // Exit with error code
    }

    let pattern = &args[1];
    let filename = &args[2];

    println!("Searching for '{}' in '{}'", pattern, filename);

    // Call run and handle errors
    if let Err(e) = run(pattern, filename) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
```

**Key Points:**
- `args[0]` is always the program name
- Use `eprintln!` for error messages (prints to stderr)
- Use `process::exit(1)` for error exit codes
- Borrow strings with `&` to avoid moving

---

### Step 2: The `run()` Function

**Convention:** Return `Result<T, E>` for functions that can fail

**When to use Result:**
- File operations (might not exist)
- Network operations (might timeout)
- Any operation that can fail

```rust
fn run(pattern: &str, filename: &str) -> Result<(), String> {
    // Read the file
    // Use ? operator to propagate errors
    let contents = fs::read_to_string(filename)
        .map_err(|e| format!("Failed to read '{}': {}", filename, e))?;

    // Search for matches
    let matches = search(pattern, &contents);

    // Display results
    display_results(filename, &matches);

    Ok(())  // Return Ok if everything succeeded
}
```

**Key Points:**
- `Result<(), String>` means: success returns nothing, error returns String
- `.map_err()` converts io::Error to String for simpler error handling
- `?` operator propagates errors up to caller
- Always return `Ok(())` at the end if success

---

### Step 3: The `search()` Function

**Convention:** Use lifetimes when returning references

**Why lifetimes?** The returned slices borrow from `contents`, so we need to tell Rust they're related.

```rust
// Lifetime 'a means: returned references live as long as contents
fn search<'a>(pattern: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut results = Vec::new();

    // Iterate over lines with line numbers
    // enumerate() gives us (index, item) tuples
    for (index, line) in contents.lines().enumerate() {
        if line.contains(pattern) {
            // Line numbers start at 1 (not 0) for users
            results.push((index + 1, line));
        }
    }

    results
}
```

**Key Points:**
- `<'a>` is a lifetime parameter
- `&'a str` means "a string slice that lives as long as 'a"
- `enumerate()` gives you (index, item) pairs
- `.lines()` splits string by newlines
- Return `Vec` of tuples: (line_number, line_text)

**Alternative: Using iterators (more idiomatic!)**

```rust
fn search<'a>(pattern: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(pattern))
        .map(|(i, line)| (i + 1, line))
        .collect()
}
```

---

### Step 4: Display Results

**Convention:** Separate display logic from business logic

```rust
fn display_results(filename: &str, matches: &[(usize, &str)]) {
    if matches.is_empty() {
        println!("No matches found");
        return;  // Early return pattern
    }

    println!("\n{}:", filename);
    for (line_num, line) in matches {
        // Print line number and trimmed line
        println!("  {}: {}", line_num, line.trim());
    }

    println!("\nFound {} match(es)", matches.len());
}
```

**Key Points:**
- Early return for empty case
- Use `&[(usize, &str)]` for slice of tuples
- `.trim()` removes leading/trailing whitespace
- Destructure tuples in for loop: `(line_num, line)`

---

## 🧪 Testing

**Convention:** Put tests in a `tests` module with `#[cfg(test)]`

```rust
#[cfg(test)]
mod tests {
    use super::*;  // Import functions from parent module

    #[test]
    fn test_search_basic() {
        let contents = "Rust is safe.\nRust is fast.\nC is old.";
        let results = search("Rust", contents);

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, 1);  // First match at line 1
        assert_eq!(results[1].0, 2);  // Second match at line 2
    }

    #[test]
    fn test_search_no_match() {
        let contents = "Hello world";
        let results = search("Rust", contents);

        assert!(results.is_empty());
    }

    #[test]
    fn test_search_case_sensitive() {
        let contents = "Rust\nrust\nRUST";
        let results = search("Rust", contents);

        // Should only match exact case
        assert_eq!(results.len(), 1);
    }
}
```

**Key Points:**
- `#[cfg(test)]` only compiles in test mode
- `use super::*` imports from parent module
- `#[test]` marks a test function
- `assert_eq!` checks equality
- `assert!` checks boolean
- Run tests with: `cargo test`

---

## 🎨 Code Style Conventions

### Error Handling
- Use `Result<T, E>` for fallible functions
- Use `?` operator to propagate errors
- Use `eprintln!` for error messages
- Use `process::exit(code)` for error exits

### Naming
- **Modules:** `snake_case` → `my_module`
- **Functions:** `snake_case` → `search_files`
- **Variables:** `snake_case` → `line_count`
- **Types:** `PascalCase` → `SearchResult`
- **Constants:** `SCREAMING_SNAKE_CASE` → `MAX_LINES`

### When to use what:
- **Vec** - When you need a growable array
- **&str** - When borrowing string data
- **String** - When you own string data
- **&[T]** - When borrowing a slice of data
- **Result** - When operation can fail
- **Option** - When value might not exist

---

## ✅ Completion Checklist

### Basic Implementation
- [ ] Parse command-line arguments
- [ ] Read file with error handling
- [ ] Search for pattern in file
- [ ] Display results with line numbers
- [ ] Exit with proper codes

### Error Handling
- [ ] Handle missing arguments
- [ ] Handle file not found
- [ ] Handle file read errors
- [ ] Print errors to stderr

### Tests
- [ ] Test with matches found
- [ ] Test with no matches
- [ ] Test case sensitivity
- [ ] All tests pass: `cargo test`

### Code Quality
- [ ] No warnings: `cargo build`
- [ ] Follows naming conventions
- [ ] Functions properly documented
- [ ] Code is readable

---

## 🚀 Testing Your Program

```bash
# Create a test file
echo -e "TODO: implement feature\nBug in line 2\nTODO: fix tests" > test.txt

# Search for TODO
cargo run TODO test.txt

# Expected output:
# Searching for 'TODO' in 'test.txt'
#
# test.txt:
#   1: TODO: implement feature
#   3: TODO: fix tests
#
# Found 2 match(es)

# Test error cases
cargo run                    # Should show usage
cargo run "pattern"          # Should show usage
cargo run "x" missing.txt    # Should show file error
```

---

## 💡 Extension Ideas

Once basic version works:

### Level 1 (Easy)
- [ ] Add `-i` flag for case-insensitive search
- [ ] Add `--count` flag to only show count
- [ ] Highlight the matched pattern in output

### Level 2 (Medium)
- [ ] Search multiple files
- [ ] Recursive directory search
- [ ] Exclude certain file types

### Level 3 (Hard)
- [ ] Add regular expression support
- [ ] Show context lines (before/after match)
- [ ] Parallel file searching with threads

---

## 📚 Key Takeaways

After this project, you should understand:

✅ **Command-line args** - `std::env::args()`
✅ **File I/O** - `fs::read_to_string()`
✅ **Error handling** - `Result`, `?` operator
✅ **Lifetimes** - `<'a>` for borrowed data
✅ **Iterators** - `.lines()`, `.enumerate()`, `.filter()`
✅ **Testing** - `#[test]`, `assert_eq!`
✅ **Code organization** - Separating concerns

---

## 📖 Usage Examples

```bash
# Basic search
cargo run "TODO" src/main.rs

# Search in different file
cargo run "fn" src/lib.rs

# Search for errors
cargo run "ERROR" logs.txt

# Run tests
cargo test

# Build release version
cargo build --release
./target/release/ch02-tour "pattern" file.txt
```

---

**Start coding!** Build it step by step, test as you go! 🔍
