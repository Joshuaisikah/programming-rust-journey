// transaction.rs — Write-Ahead Log (WAL) based transactions
//
// Provides begin/commit/rollback semantics:
//   - Operations are buffered in a Transaction
//   - commit() applies them atomically to the collection
//   - rollback() discards all buffered ops

use std::collections::HashMap;
use serde_json::Value;
use crate::document::DocumentId;

/// A single operation buffered in a transaction.
#[derive(Debug, Clone)]
pub enum Operation {
    Insert(HashMap<String, Value>),
    Update(DocumentId, HashMap<String, Value>),
    Delete(DocumentId),
}

/// An in-progress transaction. Collects operations until committed or rolled back.
pub struct Transaction {
    ops: Vec<Operation>,
    committed: bool,
}

impl Transaction {
    /// Begin a new transaction.
    pub fn begin() -> Self {
        todo!("Transaction {{ ops: vec![], committed: false }}")
    }

    /// Buffer an insert operation.
    pub fn insert(&mut self, fields: HashMap<String, Value>) {
        todo!("push Operation::Insert(fields)")
    }

    /// Buffer an update operation.
    pub fn update(&mut self, id: DocumentId, fields: HashMap<String, Value>) {
        todo!("push Operation::Update(id, fields)")
    }

    /// Buffer a delete operation.
    pub fn delete(&mut self, id: DocumentId) {
        todo!("push Operation::Delete(id)")
    }

    /// Return all buffered operations without committing.
    /// Used by Collection to apply them atomically.
    pub fn operations(&self) -> &[Operation] {
        todo!("&self.ops")
    }

    /// Mark this transaction as committed.
    pub fn commit(&mut self) {
        todo!("self.committed = true")
    }

    /// Discard all buffered operations.
    pub fn rollback(&mut self) {
        todo!("self.ops.clear()")
    }

    /// True if this transaction has been committed.
    pub fn is_committed(&self) -> bool {
        todo!("self.committed")
    }

    /// Number of buffered operations.
    pub fn len(&self) -> usize {
        todo!("self.ops.len()")
    }

    pub fn is_empty(&self) -> bool {
        todo!("self.ops.is_empty()")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn fields(k: &str, v: i64) -> HashMap<String, Value> {
        let mut m = HashMap::new();
        m.insert(k.to_string(), json!(v));
        m
    }

    #[test]
    #[ignore = "implement Transaction::begin"]
    fn test_new_transaction_is_empty() {
        let tx = Transaction::begin();
        assert!(tx.is_empty());
        assert!(!tx.is_committed());
    }

    #[test]
    #[ignore = "implement Transaction::insert / len"]
    fn test_insert_buffers_operation() {
        let mut tx = Transaction::begin();
        tx.insert(fields("score", 100));
        assert_eq!(tx.len(), 1);
    }

    #[test]
    #[ignore = "implement Transaction::commit / is_committed"]
    fn test_commit_marks_transaction() {
        let mut tx = Transaction::begin();
        tx.insert(fields("x", 1));
        tx.commit();
        assert!(tx.is_committed());
    }

    #[test]
    #[ignore = "implement Transaction::rollback"]
    fn test_rollback_clears_ops() {
        let mut tx = Transaction::begin();
        tx.insert(fields("x", 1));
        tx.delete(5);
        tx.rollback();
        assert!(tx.is_empty());
        assert!(!tx.is_committed());
    }

    #[test]
    #[ignore = "implement Transaction::operations"]
    fn test_operations_returns_all_buffered() {
        let mut tx = Transaction::begin();
        tx.insert(fields("a", 1));
        tx.update(2, fields("b", 2));
        tx.delete(3);
        assert_eq!(tx.operations().len(), 3);
    }
}
