// index.rs — Field index for fast lookup
//
// Maps field values → sets of document IDs.
// Backed by a BTreeMap for sorted iteration.
// Only supports indexing a single field per Index instance.

use std::collections::{BTreeMap, HashSet};
use serde_json::Value;
use crate::document::DocumentId;

pub struct Index {
    field: String,
    /// value (serialized to string key) → set of doc IDs
    entries: BTreeMap<String, HashSet<DocumentId>>,
}

impl Index {
    /// Create an index for the given field name.
    pub fn new(field: impl Into<String>) -> Self {
        todo!("Index {{ field: field.into(), entries: BTreeMap::new() }}")
    }

    /// The name of the field this index covers.
    pub fn field(&self) -> &str {
        todo!("&self.field")
    }

    /// Add a document to the index under the given field value.
    pub fn insert(&mut self, value: &Value, doc_id: DocumentId) {
        todo!("serialize value to string key, insert doc_id into the set")
    }

    /// Remove a document from the index for the given value.
    pub fn remove(&mut self, value: &Value, doc_id: DocumentId) {
        todo!("find entry, remove doc_id from set, remove entry if empty")
    }

    /// Return all document IDs that have the given field value.
    pub fn lookup(&self, value: &Value) -> HashSet<DocumentId> {
        todo!("serialize value, look up in entries, clone the set or return empty")
    }

    /// Total number of unique indexed values.
    pub fn cardinality(&self) -> usize {
        todo!("self.entries.len()")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    #[ignore = "implement Index::new / field"]
    fn test_new_stores_field_name() {
        let idx = Index::new("age");
        assert_eq!(idx.field(), "age");
        assert_eq!(idx.cardinality(), 0);
    }

    #[test]
    #[ignore = "implement Index::insert / lookup"]
    fn test_insert_and_lookup() {
        let mut idx = Index::new("country");
        idx.insert(&json!("US"), 1);
        idx.insert(&json!("US"), 2);
        idx.insert(&json!("UK"), 3);

        let us_ids = idx.lookup(&json!("US"));
        assert!(us_ids.contains(&1));
        assert!(us_ids.contains(&2));
        assert!(!us_ids.contains(&3));
    }

    #[test]
    #[ignore = "implement Index::lookup"]
    fn test_lookup_missing_value_returns_empty_set() {
        let idx = Index::new("name");
        let result = idx.lookup(&json!("ghost"));
        assert!(result.is_empty());
    }

    #[test]
    #[ignore = "implement Index::remove"]
    fn test_remove_doc_from_index() {
        let mut idx = Index::new("tag");
        idx.insert(&json!("rust"), 10);
        idx.insert(&json!("rust"), 11);
        idx.remove(&json!("rust"), 10);

        let ids = idx.lookup(&json!("rust"));
        assert!(!ids.contains(&10));
        assert!(ids.contains(&11));
    }

    #[test]
    #[ignore = "implement Index::cardinality"]
    fn test_cardinality() {
        let mut idx = Index::new("status");
        idx.insert(&json!("active"), 1);
        idx.insert(&json!("inactive"), 2);
        idx.insert(&json!("active"), 3); // duplicate value key
        assert_eq!(idx.cardinality(), 2); // only 2 unique values
    }
}
