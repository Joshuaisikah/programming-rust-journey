// ============================================================
// main.rs — Entry Point
// ============================================================
//
// USAGE:
//   cargo run -- moves   → Move semantics demo
//   cargo run -- copies  → Copy vs Clone demo
//   cargo run -- heap    → Box / Rc / Arc demo
//   cargo run -- all     → Run all demos

use std::env;
use ch04_ownership::{demo_copies, demo_heap, demo_moves};
use ch04_ownership::display::print_usage;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "moves"  => demo_moves(),
        "copies" => demo_copies(),
        "heap"   => demo_heap(),
        "all"    => {
            demo_moves();
            demo_copies();
            demo_heap();
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
        }
    }
}
