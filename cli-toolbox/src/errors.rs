// Ch07 — Error Handling
//
// CONCEPTS:
//   thiserror::Error — derive macro that generates Display + Error impls
//   #[from]          — auto-converts an underlying error into this type
//   #[error("...")]  — the Display string for each variant
//   ? operator       — propagates Err, unwrapping Ok (requires From impl)

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("task not found: '{0}'")]
    NotFound(String),

    #[error("invalid priority '{0}': expected high, medium, or low")]
    InvalidPriority(String),

    #[error("invalid command: {0}")]
    InvalidCommand(String),

    #[error("invalid id '{0}': expected a positive integer")]
    InvalidId(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json error: {0}")]
    Json(String),
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_found_message() {
        let e = CliError::NotFound("buy milk".into());
        assert_eq!(e.to_string(), "task not found: 'buy milk'");
    }

    #[test]
    fn test_invalid_priority_message() {
        let e = CliError::InvalidPriority("urgent".into());
        assert_eq!(e.to_string(), "invalid priority 'urgent': expected high, medium, or low");
    }

    #[test]
    fn test_invalid_id_message() {
        let e = CliError::InvalidId("abc".into());
        assert_eq!(e.to_string(), "invalid id 'abc': expected a positive integer");
    }

    #[test]
    fn test_io_error_converts_via_from() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let cli_err = CliError::from(io_err);
        assert!(cli_err.to_string().contains("io error"));
    }

    #[test]
    fn test_invalid_command_message() {
        let e = CliError::InvalidCommand("unknown subcommand 'fly'".into());
        assert_eq!(e.to_string(), "invalid command: unknown subcommand 'fly'");
    }
}
