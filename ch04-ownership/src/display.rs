/// Prints CLI usage instructions.
pub fn print_usage() {
    println!("Usage:");
    println!("  cargo run -- moves   # Move semantics demo");
    println!("  cargo run -- copies  # Copy vs Clone demo");
    println!("  cargo run -- heap    # Box / Rc / Arc demo");
    println!("  cargo run -- all     # Run all demos");
}

pub fn section(title: &str) {
    println!("\n{}", "=".repeat(50));
    println!("  {}", title);
    println!("{}", "=".repeat(50));
}

#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {
        assert!(true);
    }
}
