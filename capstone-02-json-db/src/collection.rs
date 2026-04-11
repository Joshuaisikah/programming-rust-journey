// collection.rs — In-memory document collection with CRUD
//
// A Collection holds documents in a HashMap<DocumentId, Document>
// plus optional indexes for fast field-based lookup.

use std::collections::HashMap;
use serde_json::Value;

use crate::document::{Document, DocumentId};
use crate::error::DbError;
use crate::index::Index;
use crate::query::Query;

pub struct Collection {
    pub name: String,
    documents: HashMap<DocumentId, Document>,
    indexes: HashMap<String, Index>,
    next_id: DocumentId,
}

impl Collection {
    /// Create a new empty collection with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        todo!("Collection {{ name: name.into(), documents: HashMap::new(), indexes: HashMap::new(), next_id: 1 }}")
    }

    /// Insert a document. The collection assigns the ID automatically.
    /// Returns the assigned DocumentId.
    pub fn insert(&mut self, fields: HashMap<String, Value>) -> Result<DocumentId, DbError> {
        todo!("create Document with next_id, add to documents, update indexes, increment next_id")
    }

    /// Insert a document with a specific ID (e.g., when loading from disk).
    /// Returns DuplicateId error if the id already exists.
    pub fn insert_with_id(&mut self, doc: Document) -> Result<(), DbError> {
        todo!("check for duplicate, insert, update indexes")
    }

    /// Get a document by ID.
    pub fn get(&self, id: DocumentId) -> Option<&Document> {
        todo!("self.documents.get(&id)")
    }

    /// Update fields of a document. Merges new fields into the existing document.
    /// Returns NotFound if the id does not exist.
    pub fn update(&mut self, id: DocumentId, fields: HashMap<String, Value>) -> Result<(), DbError> {
        todo!("find doc, remove from indexes, update fields, re-index")
    }

    /// Delete a document by ID.
    pub fn delete(&mut self, id: DocumentId) -> Result<Document, DbError> {
        todo!("remove from documents and indexes, return removed doc")
    }

    /// Find all documents matching the query.
    pub fn find(&self, query: &Query) -> Vec<&Document> {
        todo!("filter documents by query.matches, then sort and apply limit/offset")
    }

    /// Total number of documents in the collection.
    pub fn count(&self) -> usize {
        todo!("self.documents.len()")
    }

    /// Create an index on the given field for faster lookups.
    pub fn create_index(&mut self, field: &str) {
        todo!("build Index from existing documents, add to self.indexes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use crate::query::{Operator, Query};

    fn make_fields(name: &str, age: i64) -> HashMap<String, Value> {
        let mut m = HashMap::new();
        m.insert("name".to_string(), json!(name));
        m.insert("age".to_string(), json!(age));
        m
    }

    #[test]
    #[ignore = "implement Collection::new"]
    fn test_new_collection_is_empty() {
        let c = Collection::new("users");
        assert_eq!(c.count(), 0);
        assert_eq!(c.name, "users");
    }

    #[test]
    #[ignore = "implement Collection::insert"]
    fn test_insert_assigns_sequential_ids() {
        let mut c = Collection::new("users");
        let id1 = c.insert(make_fields("Alice", 30)).unwrap();
        let id2 = c.insert(make_fields("Bob", 25)).unwrap();
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(c.count(), 2);
    }

    #[test]
    #[ignore = "implement Collection::get"]
    fn test_get_returns_inserted_doc() {
        let mut c = Collection::new("users");
        let id = c.insert(make_fields("Alice", 30)).unwrap();
        let doc = c.get(id).unwrap();
        assert_eq!(doc.get("name"), Some(&json!("Alice")));
    }

    #[test]
    #[ignore = "implement Collection::get"]
    fn test_get_missing_returns_none() {
        let c = Collection::new("users");
        assert!(c.get(999).is_none());
    }

    #[test]
    #[ignore = "implement Collection::update"]
    fn test_update_merges_fields() {
        let mut c = Collection::new("users");
        let id = c.insert(make_fields("Alice", 30)).unwrap();
        let mut patch = HashMap::new();
        patch.insert("age".to_string(), json!(31));
        c.update(id, patch).unwrap();
        assert_eq!(c.get(id).unwrap().get("age"), Some(&json!(31)));
        assert_eq!(c.get(id).unwrap().get("name"), Some(&json!("Alice")));
    }

    #[test]
    #[ignore = "implement Collection::update"]
    fn test_update_not_found_errors() {
        let mut c = Collection::new("users");
        let mut patch = HashMap::new();
        patch.insert("age".to_string(), json!(31));
        assert!(c.update(999, patch).is_err());
    }

    #[test]
    #[ignore = "implement Collection::delete"]
    fn test_delete_removes_doc() {
        let mut c = Collection::new("users");
        let id = c.insert(make_fields("Alice", 30)).unwrap();
        c.delete(id).unwrap();
        assert_eq!(c.count(), 0);
        assert!(c.get(id).is_none());
    }

    #[test]
    #[ignore = "implement Collection::delete"]
    fn test_delete_not_found_errors() {
        let mut c = Collection::new("users");
        assert!(c.delete(999).is_err());
    }

    #[test]
    #[ignore = "implement Collection::find"]
    fn test_find_with_filter() {
        let mut c = Collection::new("users");
        c.insert(make_fields("Alice", 30)).unwrap();
        c.insert(make_fields("Bob", 17)).unwrap();
        c.insert(make_fields("Carol", 25)).unwrap();

        let q = Query::new().filter("age", Operator::Gte, json!(18));
        let results = c.find(&q);
        assert_eq!(results.len(), 2); // Alice and Carol
    }

    #[test]
    #[ignore = "implement Collection::find"]
    fn test_find_with_limit() {
        let mut c = Collection::new("users");
        for i in 0..10 {
            c.insert(make_fields(&format!("User{}", i), i as i64 * 10)).unwrap();
        }
        let q = Query::new().limit(3);
        let results = c.find(&q);
        assert_eq!(results.len(), 3);
    }
}
