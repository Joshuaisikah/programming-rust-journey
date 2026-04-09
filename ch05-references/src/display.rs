/// Prints CLI usage instructions.
pub fn print_usage() {
    println!("Usage:");
    println!("  cargo run -- borrows    # Shared & mutable reference demos");
    println!("  cargo run -- lifetimes  # Lifetime annotation demos");
    println!("  cargo run -- slices     # Slice demos");
    println!("  cargo run -- all        # Run all demos");
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
