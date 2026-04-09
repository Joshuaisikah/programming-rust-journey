// ============================================================
// main.rs — Entry Point
// ============================================================
//
// USAGE:
//   cargo run -- borrows    → Shared & mutable reference demos
//   cargo run -- lifetimes  → Lifetime annotation demos
//   cargo run -- slices     → Slice demos
//   cargo run -- all        → Run all demos

use std::env;
use ch05_references::{demo_borrows, demo_lifetimes, demo_slices};
use ch05_references::display::print_usage;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "borrows"   => demo_borrows(),
        "lifetimes" => demo_lifetimes(),
        "slices"    => demo_slices(),
        "all"       => {
            demo_borrows();
            demo_lifetimes();
            demo_slices();
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
        }
    }
}
