use std::env;
use std::process;
use ch02_tour::{Config, run};
use ch02_tour::display::{display_error, display_usage};

fn main() {
    let args = env::args();
    let config = Config::new(args).unwrap_or_else(|err| {
        display_error(&err);
        display_usage(&env::args().next().unwrap_or_else(|| "minigrep".to_string()));
        process::exit(1);  // Exit with error code (like Environment.Exit(1) in C#)
    });
    println!("Searching for '{}' in '{}'", config.pattern, config.filename);
    if let Err(e) = run(config) {
        display_error(&e.to_string());
        process::exit(1);
    }
}

// NOTE: No tests in main.rs
// 🎓 C# DEVELOPERS:you are  In C#, you might have integration tests
// In Rust, put tests in lib.rs and module files
// main.rs is just the entry point - keep it minimal!

// 🎓 KEY PATTERNS IN MAIN:
// 1. unwrap_or_else: Handle errors with fallback logic
// 2. if let Err(e): Pattern match on error case only
// 3. process::exit(code): Exit with error code (like Environment.Exit)
// 4. Keep main.rs thin - all logic in lib.rs for testability
