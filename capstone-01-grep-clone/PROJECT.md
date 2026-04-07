# Capstone 1: Fast Grep Clone (Like ripgrep)

Build a production-quality file search tool that's faster than standard grep.

**Chapters Combined:** 7 (Error Handling), 14 (Closures), 15 (Iterators), 17 (Strings), 18 (I/O)

---

## 🎯 Learning Goals

- Handle errors elegantly across complex operations
- Use closures for filters and callbacks
- Chain iterators for performance
- Process text efficiently
- Implement buffered I/O
- Build a real CLI tool with clap

---

## 📋 Project Structure

```rust
// Project organization:

src/
├── main.rs           // CLI entry point
├── lib.rs            // Library code
├── search.rs         // Search algorithms
├── walker.rs         // Directory traversal
├── matcher.rs        // Pattern matching
└── error.rs          // Custom error types

// Key modules:

mod search {
    pub fn search_file(path: &Path, pattern: &str) -> Result<Vec<Match>>;
    pub fn search_directory(dir: &Path, pattern: &str, recursive: bool) -> Results;
}

mod matcher {
    pub trait Matcher {
        fn is_match(&self, line: &str) -> bool;
    }

    pub struct LiteralMatcher { pattern: String }
    pub struct RegexMatcher { regex: Regex }
}

mod walker {
    pub fn walk_directory(path: &Path, ignore: &GitIgnore) -> impl Iterator<Item = PathBuf>;
}
```

---

## 🔨 Implementation Guide

### Part 1: Custom Error Type

**Convention:** Use `thiserror` for ergonomic error handling

```rust
// error.rs
use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GrepError {
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),

    #[error("Invalid regex pattern: {0}")]
    InvalidPattern(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, GrepError>;
```

**Key Points:**
- `#[derive(Error)]` from thiserror crate
- `#[error("...")]` defines display message
- `#[from]` auto-converts from io::Error
- Custom Result type for convenience

---

### Part 2: Search Function with Iterators

**Convention:** Return iterators for lazy evaluation (performance!)

```rust
// search.rs
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Match {
    pub line_number: usize,
    pub line_content: String,
    pub file_path: PathBuf,
}

pub fn search_file<P: AsRef<Path>>(
    path: P,
    pattern: &str,
    case_insensitive: bool,
) -> Result<Vec<Match>> {
    let file = File::open(path.as_ref())?;
    let reader = BufReader::new(file);  // Buffered for performance!

    let pattern_lower = if case_insensitive {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };

    let matches: Vec<Match> = reader
        .lines()
        .enumerate()
        .filter_map(|(index, line_result)| {
            let line = line_result.ok()?;  // Skip bad lines

            let matches = if case_insensitive {
                line.to_lowercase().contains(&pattern_lower)
            } else {
                line.contains(pattern)
            };

            if matches {
                Some(Match {
                    line_number: index + 1,
                    line_content: line,
                    file_path: path.as_ref().to_path_buf(),
                })
            } else {
                None
            }
        })
        .collect();

    Ok(matches)
}
```

**Key Points:**
- `BufReader` for efficient reading (reads in chunks, not byte-by-byte)
- `enumerate()` gives line numbers
- `filter_map()` combines filter and map
- `?` in closure returns None on error
- Generic `P: AsRef<Path>` accepts &str, String, Path, PathBuf

---

### Part 3: Trait for Different Matchers

**Convention:** Use traits for extensibility

```rust
// matcher.rs
pub trait Matcher: Send + Sync {
    fn is_match(&self, text: &str) -> bool;
}

pub struct LiteralMatcher {
    pattern: String,
    case_insensitive: bool,
}

impl LiteralMatcher {
    pub fn new(pattern: String, case_insensitive: bool) -> Self {
        Self {
            pattern: if case_insensitive {
                pattern.to_lowercase()
            } else {
                pattern
            },
            case_insensitive,
        }
    }
}

impl Matcher for LiteralMatcher {
    fn is_match(&self, text: &str) -> bool {
        if self.case_insensitive {
            text.to_lowercase().contains(&self.pattern)
        } else {
            text.contains(&self.pattern)
        }
    }
}

// For regex support (extension)
pub struct RegexMatcher {
    regex: regex::Regex,
}

impl Matcher for RegexMatcher {
    fn is_match(&self, text: &str) -> bool {
        self.regex.is_match(text)
    }
}
```

**Key Points:**
- `Send + Sync` allows use in multiple threads
- Separate matchers for different strategies
- Can add new matchers without changing search code

---

### Part 4: Directory Walker

**Convention:** Use `walkdir` crate or implement with `fs::read_dir`

```rust
// walker.rs
use std::fs;
use std::path::{Path, PathBuf};

pub struct Walker {
    recursive: bool,
    ignore_hidden: bool,
}

impl Walker {
    pub fn new(recursive: bool) -> Self {
        Self {
            recursive,
            ignore_hidden: true,
        }
    }

    pub fn walk(&self, root: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();
        self.walk_dir(root, &mut files);
        files
    }

    fn walk_dir(&self, dir: &Path, files: &mut Vec<PathBuf>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();

                // Skip hidden files if configured
                if self.ignore_hidden && is_hidden(&path) {
                    continue;
                }

                if path.is_file() {
                    files.push(path);
                } else if path.is_dir() && self.recursive {
                    self.walk_dir(&path, files);
                }
            }
        }
    }
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
```

**Key Points:**
- Recursive directory traversal
- Skip hidden files (`.git`, etc.)
- Filter bad entries gracefully
- Return Vec or Iterator based on needs

---

### Part 5: CLI with clap

**Convention:** Use `clap` derive API for ergonomic CLI

```rust
// main.rs
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "rgrep")]
#[command(about = "A fast grep clone in Rust", long_about = None)]
struct Args {
    /// Pattern to search for
    pattern: String,

    /// File or directory to search
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Case insensitive search
    #[arg(short, long)]
    ignore_case: bool,

    /// Search recursively in directories
    #[arg(short, long)]
    recursive: bool,

    /// Only show count of matches
    #[arg(short, long)]
    count: bool,

    /// Use regex pattern
    #[arg(long)]
    regex: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Create matcher
    let matcher: Box<dyn Matcher> = if args.regex {
        Box::new(RegexMatcher::new(&args.pattern)?)
    } else {
        Box::new(LiteralMatcher::new(args.pattern.clone(), args.ignore_case))
    };

    // Search
    if args.path.is_file() {
        search_file_and_print(&args.path, &*matcher, args.count)?;
    } else {
        search_directory_and_print(&args.path, &*matcher, args.recursive, args.count)?;
    }

    Ok(())
}
```

**Key Points:**
- `#[derive(Parser)]` from clap
- `#[arg(short, long)]` creates `-i` and `--ignore-case`
- Box<dyn Trait> for trait objects (runtime polymorphism)
- Clean separation: parsing, logic, display

---

### Part 6: Parallel Search (Performance!)

**Convention:** Use `rayon` for easy parallelism

```rust
use rayon::prelude::*;

pub fn search_files_parallel(
    files: Vec<PathBuf>,
    matcher: &dyn Matcher,
) -> Vec<Match> {
    files
        .par_iter()  // Parallel iterator!
        .flat_map(|path| {
            search_file(path, matcher)
                .unwrap_or_default()
        })
        .collect()
}
```

**Key Points:**
- `.par_iter()` makes it parallel (that's it!)
- rayon handles thread pool automatically
- Matcher needs `Send + Sync` (which we added!)

---

## 📦 Dependencies (Cargo.toml)

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
regex = "1.11"
rayon = "1.10"
thiserror = "2.0"
anyhow = "1.0"

[dev-dependencies]
criterion = "0.5"  # For benchmarking
```

---

## 🧪 Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_search_finds_matches() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("test.txt");

        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "Hello World").unwrap();
        writeln!(file, "Goodbye World").unwrap();
        writeln!(file, "Rust is awesome").unwrap();

        let matcher = LiteralMatcher::new("World".to_string(), false);
        let matches = search_file(&file_path, &matcher).unwrap();

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].line_number, 1);
        assert_eq!(matches[1].line_number, 2);
    }

    #[test]
    fn test_case_insensitive() {
        // Create temp file with mixed case
        // Test that case_insensitive flag works
    }

    #[test]
    fn test_recursive_search() {
        // Create temp directory structure
        // Test recursive traversal
    }
}
```

---

## 🎨 Code Conventions

### Error Handling Strategy
- Use `Result<T, GrepError>` everywhere
- Use `?` operator for propagation
- Convert errors at boundaries (io::Error → GrepError)
- Don't panic! Return errors

### Performance Guidelines
- Use `BufReader` for files
- Use iterators (lazy evaluation)
- Use `rayon` for parallelism
- Avoid allocations in hot paths
- Profile with `criterion`

### When to use what:
- **Trait objects (`Box<dyn Trait>`)** - Runtime polymorphism
- **Generics (`<T: Trait>`)** - Compile-time polymorphism (faster!)
- **Closures** - Filters, callbacks
- **Iterators** - Processing sequences

---

## ✅ Completion Checklist

### Core Features
- [ ] Search single file for pattern
- [ ] Display matches with line numbers
- [ ] Case-insensitive search
- [ ] Recursive directory search
- [ ] Count-only mode
- [ ] Ignore hidden files/directories

### Error Handling
- [ ] Custom error type
- [ ] Graceful file errors
- [ ] Clear error messages
- [ ] Exit codes

### Performance
- [ ] Buffered I/O
- [ ] Parallel search with rayon
- [ ] Benchmark against grep

### CLI
- [ ] Help message
- [ ] Multiple flags
- [ ] Good UX

### Code Quality
- [ ] Tests pass
- [ ] No warnings
- [ ] Documentation
- [ ] Follows conventions

---

## 🚀 Usage Examples

```bash
# Basic search
cargo run -- "TODO" src/

# Case insensitive
cargo run -- -i "error" logs/

# Recursive search
cargo run -- -r "fn main" .

# Count only
cargo run -- -c "import" src/

# With regex
cargo run -- --regex "fn \w+\(" src/

# Build release (fast!)
cargo build --release
./target/release/rgrep "pattern" /path
```

---

## 📊 Benchmarking

```bash
# Install criterion
cargo add --dev criterion

# Run benchmarks
cargo bench

# Compare to system grep
time grep -r "pattern" /path
time ./target/release/rgrep "pattern" /path
```

---

## 💡 Extension Ideas

### Level 1 (Medium)
- [ ] Colored output (highlight matches)
- [ ] Context lines (show N lines before/after)
- [ ] .gitignore support
- [ ] Binary file detection

### Level 2 (Hard)
- [ ] Memory-mapped files for huge files
- [ ] Compressed file support (.gz)
- [ ] Replace mode (find and replace)
- [ ] JSON output mode

### Level 3 (Expert)
- [ ] SIMD optimizations
- [ ] Custom regex engine
- [ ] Distributed search
- [ ] Interactive mode (TUI)

---

## 📚 Key Takeaways

After this capstone:

✅ **Error handling at scale** - Custom errors, graceful degradation
✅ **High-performance I/O** - Buffered reading, memory efficiency
✅ **Parallel processing** - rayon for easy speedups
✅ **Trait objects** - Runtime polymorphism
✅ **Iterator chains** - Functional, efficient processing
✅ **Production CLI** - clap, good UX, proper exit codes

---

## 🎯 Success Criteria

Your grep clone should:
- ✅ Handle 1GB+ files
- ✅ Search 10,000+ files in seconds
- ✅ Never panic (only return errors)
- ✅ Use < 100MB memory
- ✅ Be faster than standard grep

**Build a tool you'd actually use!** 🚀
