// ============================================================
// main.rs — Demo runner
// ============================================================
//
// USAGE:
//   cargo run -- stack        → Stack<T> demo
//   cargo run -- matrix       → Matrix operator demo
//   cargo run -- functional   → Closure patterns demo
//   cargo run -- iterators    → Custom iterators demo
//   cargo run -- collections  → Collection patterns demo

use std::env;

use datastruct_lib::{collections, functional, iterators, matrix, stack};

fn print_usage() {
    println!("Usage:");
    println!("  cargo run -- stack");
    println!("  cargo run -- matrix");
    println!("  cargo run -- functional");
    println!("  cargo run -- iterators");
    println!("  cargo run -- collections");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "stack"       => stack::demo(),
        "matrix"      => matrix::demo(),
        "functional"  => functional::demo(),
        "iterators"   => iterators::demo(),
        "collections" => collections::demo(),
        _ => {
            eprintln!("Unknown: {}", args[1]);
            print_usage();
        }
    }
}
