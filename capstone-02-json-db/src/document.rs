// document.rs — Document model
//
// A Document is a JSON object with a unique ID.
// Wraps serde_json::Value so any valid JSON can be stored.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub type DocumentId = u64;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Document {
    #[serde(rename = "_id")]
    pub id: DocumentId,
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

impl Document {
    /// Create a new document with the given id and field map.
    pub fn new(id: DocumentId, fields: HashMap<String, Value>) -> Self {
        todo!("construct Document {{ id, fields }}")
    }

    /// Create a document from a JSON object string, assigning the given id.
    /// Returns an error if the string is not a valid JSON object.
    pub fn from_json(id: DocumentId, json: &str) -> Result<Self, serde_json::Error> {
        todo!("parse json, extract object fields, attach id")
    }

    /// Get the value of a field by name.
    pub fn get(&self, field: &str) -> Option<&Value> {
        todo!("self.fields.get(field)")
    }

    /// Set or overwrite a field.
    pub fn set(&mut self, field: &str, value: Value) {
        todo!("self.fields.insert(field.to_string(), value)")
    }

    /// Remove a field. Returns the old value if it existed.
    pub fn remove(&mut self, field: &str) -> Option<Value> {
        todo!("self.fields.remove(field)")
    }

    /// Serialize this document to a JSON string.
    pub fn to_json(&self) -> String {
        todo!("serde_json::to_string(self).unwrap()")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn doc(id: DocumentId) -> Document {
        let mut fields = HashMap::new();
        fields.insert("name".to_string(), json!("Alice"));
        fields.insert("age".to_string(), json!(30));
        Document::new(id, fields)
    }

    #[test]
    #[ignore = "implement Document::new"]
    fn test_new_stores_id_and_fields() {
        let d = doc(1);
        assert_eq!(d.id, 1);
        assert!(d.fields.contains_key("name"));
    }

    #[test]
    #[ignore = "implement Document::get"]
    fn test_get_existing_field() {
        let d = doc(1);
        assert_eq!(d.get("name"), Some(&json!("Alice")));
    }

    #[test]
    #[ignore = "implement Document::get"]
    fn test_get_missing_field_returns_none() {
        let d = doc(1);
        assert_eq!(d.get("email"), None);
    }

    #[test]
    #[ignore = "implement Document::set"]
    fn test_set_overwrites_field() {
        let mut d = doc(1);
        d.set("age", json!(31));
        assert_eq!(d.get("age"), Some(&json!(31)));
    }

    #[test]
    #[ignore = "implement Document::set"]
    fn test_set_new_field() {
        let mut d = doc(1);
        d.set("email", json!("alice@example.com"));
        assert!(d.get("email").is_some());
    }

    #[test]
    #[ignore = "implement Document::remove"]
    fn test_remove_existing_field() {
        let mut d = doc(1);
        let old = d.remove("name");
        assert_eq!(old, Some(json!("Alice")));
        assert!(d.get("name").is_none());
    }

    #[test]
    #[ignore = "implement Document::from_json"]
    fn test_from_json_valid() {
        let d = Document::from_json(5, r#"{"name": "Bob", "score": 99}"#).unwrap();
        assert_eq!(d.id, 5);
        assert_eq!(d.get("name"), Some(&json!("Bob")));
    }

    #[test]
    #[ignore = "implement Document::from_json"]
    fn test_from_json_invalid_returns_error() {
        assert!(Document::from_json(1, "not json").is_err());
    }

    #[test]
    #[ignore = "implement Document::to_json"]
    fn test_to_json_round_trips() {
        let d = doc(42);
        let json_str = d.to_json();
        // Must contain the id field
        assert!(json_str.contains("\"_id\""));
        assert!(json_str.contains("42"));
    }
}
