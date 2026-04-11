// error.rs — HTTP server error types

use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Route not found: {method} {path}")]
    NotFound { method: String, path: String },

    #[error("Method not allowed")]
    MethodNotAllowed,

    #[error("Handler error: {0}")]
    Handler(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_found_message_includes_path() {
        let e = HttpError::NotFound {
            method: "GET".into(),
            path: "/users".into(),
        };
        assert!(e.to_string().contains("/users"));
        assert!(e.to_string().contains("GET"));
    }
}
