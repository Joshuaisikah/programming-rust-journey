use clap::Parser;
use grep_clone::{Config, run};

fn main() {
    let config = Config::parse();
    match run(&config) {
        Ok(count) => {
            if count == 0 {
                std::process::exit(1); // grep convention: exit 1 when no matches
            }
        }
        Err(e) => {
            eprintln!("rgrep: {}", e);
            std::process::exit(2);
        }
    }
}
