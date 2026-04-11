// error.rs — Database error types

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Document with id {0} not found")]
    NotFound(u64),

    #[error("Document with id {0} already exists")]
    DuplicateId(u64),

    #[error("Invalid query: {0}")]
    InvalidQuery(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Transaction aborted: {0}")]
    TransactionAborted(String),

    #[error("Index error: {0}")]
    IndexError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_found_error_message() {
        let e = DbError::NotFound(42);
        assert!(e.to_string().contains("42"));
    }

    #[test]
    fn test_duplicate_id_error_message() {
        let e = DbError::DuplicateId(7);
        assert!(e.to_string().contains("7"));
    }

    #[test]
    fn test_invalid_query_error_message() {
        let e = DbError::InvalidQuery("bad field".to_string());
        assert!(e.to_string().contains("bad field"));
    }
}
