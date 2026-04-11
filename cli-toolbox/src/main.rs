// ============================================================
// main.rs — Entry Point
// ============================================================
//
// USAGE:
//   cargo run -- add "buy milk" high
//   cargo run -- list
//   cargo run -- list --all
//   cargo run -- done 1
//   cargo run -- delete 2
//   cargo run -- search milk

use std::env;
use cli_toolbox::{execute, parse_command};
use cli_toolbox::display::print_usage;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        print_usage();
        return;
    }

    // In a real app the task list would be persisted (Ch18 I/O).
    // For now it lives only for the duration of this run.
    let mut tasks = vec![];

    match parse_command(&args) {
        Ok(cmd) => match execute(cmd, &mut tasks) {
            Ok(msg)  => println!("{}", msg),
            Err(e)   => eprintln!("Error: {}", e),
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            print_usage();
        }
    }
}
