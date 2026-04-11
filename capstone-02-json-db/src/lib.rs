// Capstone 2 — JSON Document Database
//
// A MongoDB-Lite style embedded document store:
//   - CRUD operations on JSON documents
//   - Simple query language (filter, sort, limit)
//   - In-memory indexes for fast lookup
//   - Persist collections to disk as .jsonl files
//   - Optional transaction support (write-ahead log)

pub mod collection;
pub mod document;
pub mod error;
pub mod index;
pub mod query;
pub mod storage;
pub mod transaction;

pub use collection::Collection;
pub use document::{Document, DocumentId};
pub use error::DbError;
pub use query::Query;

/// A handle to an on-disk database directory.
pub struct Database {
    path: std::path::PathBuf,
}

impl Database {
    /// Open (or create) a database at the given directory path.
    pub fn open(path: impl Into<std::path::PathBuf>) -> Result<Self, DbError> {
        todo!("create dir if missing, return handle")
    }

    /// Get or create a named collection inside this database.
    pub fn collection(&self, name: &str) -> Result<Collection, DbError> {
        todo!("load collection from disk, or create empty one")
    }
}
