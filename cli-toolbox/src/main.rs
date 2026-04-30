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
use cli_toolbox::storage::{load_tasks, save_tasks};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        print_usage();
        return;
    }

    // Load tasks from disk (tasks.json next to where you run the binary).
    // Returns an empty list on first run.
    let mut tasks = match load_tasks() {
        Ok(t)  => t,
        Err(e) => { eprintln!("Error loading tasks: {}", e); return; }
    };

    match parse_command(&args) {
        Ok(cmd) => match execute(cmd, &mut tasks) {
            Ok(msg) => {
                // Persist the updated list before printing so the user sees
                // the save result first (any I/O error surfaces immediately).
                if let Err(e) = save_tasks(&tasks) {
                    eprintln!("Error saving tasks: {}", e);
                    return;
                }
                println!("{}", msg);
            }
            Err(e) => eprintln!("Error: {}", e),
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            print_usage();
        }
    }
}
