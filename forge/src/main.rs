// forge — Ch17 Strings/Text | Ch18 I/O | Ch19 Concurrency | Ch20 Async
//
// USAGE:
//   cargo run -- strings      → Ch17: &str, String, Unicode, text ops
//   cargo run -- io           → Ch18: BufRead, Write, buffered I/O
//   cargo run -- concurrency  → Ch19: threads, Arc<Mutex<T>>, channels
//   cargo run -- async        → Ch20: async/await, tokio::spawn, join!

mod strings;
mod io;
mod concurrency;
mod async_ops;

use std::env;

fn print_usage() {
    println!("Usage:");
    println!("  cargo run -- strings");
    println!("  cargo run -- io");
    println!("  cargo run -- concurrency");
    println!("  cargo run -- async");
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "strings" => {
            // TODO: call your strings functions here and print results
            // Available functions:
            //   strings::word_frequency(text)
            //   strings::is_palindrome(s)
            //   strings::caesar_cipher(text, shift)
            //   strings::longest_common_prefix(words)
        }
        "io" => {
            // TODO: call your io functions here
            // Available functions:
            //   io::count_lines(reader)
            //   io::read_words(reader)
            //   io::copy_uppercased(reader, writer)
            //   io::write_numbered(lines, writer)
            //
            // Tip: use std::io::Cursor::new("...") to create an in-memory reader
        }
        "concurrency" => {
            // TODO: call your concurrency functions here
            // Available functions:
            //   concurrency::parallel_sum(numbers)
            //   concurrency::concurrent_map(items, f)
            //   concurrency::roundtrip_channel(values)
            //   concurrency::shared_counter(n)
        }
        "async" => {
            // TODO: call your async functions here — use .await on each call
            // Available functions:
            //   async_ops::delayed(value, millis).await
            //   async_ops::join_all(handles).await
            //   async_ops::concurrent_add(a, b).await
            //   async_ops::spawn_n(n).await
        }
        _ => {
            eprintln!("Unknown: {}", args[1]);
            print_usage();
        }
    }
}
