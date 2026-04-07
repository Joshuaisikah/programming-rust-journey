
use std::fs;

pub mod config;    // Tells Rust: "config.rs is a module"
pub mod search;    // Makes the search module public
pub mod display;   // Like having public classes
pub mod error;

pub use config::Config;
pub use error::AppError;

pub fn run(config: Config) -> Result<(), AppError> {
    let contents = fs::read_to_string(&config.filename)?;
    let matches = if config.case_sensitive {
        search::search(&config.pattern, &contents)
    } else {
        search::search_case_insensitive(&config.pattern, &contents)
    };
    display::display_results(&config.filename, &matches);
    Ok(())
}

