// query.rs — Query DSL
//
// A Query describes a filter + sort + limit to apply to a Collection.
// Example:
//   Query::new()
//       .filter("age", Operator::Gt, json!(18))
//       .sort_by("name", SortOrder::Asc)
//       .limit(10)

use serde_json::Value;
use crate::document::Document;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    Contains, // for string fields
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub field: String,
    pub op: Operator,
    pub value: Value,
}

#[derive(Debug, Clone, Default)]
pub struct Query {
    pub filters: Vec<Filter>,
    pub sort_field: Option<String>,
    pub sort_order: SortOrder,
    pub limit: Option<usize>,
    pub offset: usize,
}

impl Default for SortOrder {
    fn default() -> Self { SortOrder::Asc }
}

impl Query {
    pub fn new() -> Self {
        Query::default()
    }

    /// Add a filter condition.
    pub fn filter(mut self, field: &str, op: Operator, value: Value) -> Self {
        todo!("push Filter into self.filters, return self")
    }

    /// Set the sort field and order.
    pub fn sort_by(mut self, field: &str, order: SortOrder) -> Self {
        todo!("set sort_field and sort_order, return self")
    }

    /// Set the maximum number of results to return.
    pub fn limit(mut self, n: usize) -> Self {
        todo!("set self.limit = Some(n), return self")
    }

    /// Skip the first `n` results.
    pub fn offset(mut self, n: usize) -> Self {
        todo!("set self.offset = n, return self")
    }

    /// Return true if `doc` satisfies all filters in this query.
    pub fn matches(&self, doc: &Document) -> bool {
        todo!("evaluate all filters against doc fields, return true if all pass")
    }
}

/// Apply a single filter to a field value.
fn evaluate_filter(doc_value: &Value, op: &Operator, query_value: &Value) -> bool {
    todo!("compare doc_value and query_value using op")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use crate::document::Document;
    use std::collections::HashMap;

    fn make_doc(id: u64, name: &str, age: i64) -> Document {
        let mut fields = HashMap::new();
        fields.insert("name".to_string(), json!(name));
        fields.insert("age".to_string(), json!(age));
        Document::new(id, fields)
    }

    // ── Query builder ─────────────────────────────────────────

    #[test]
    #[ignore = "implement Query::filter"]
    fn test_filter_builder_adds_filter() {
        let q = Query::new().filter("age", Operator::Gt, json!(18));
        assert_eq!(q.filters.len(), 1);
        assert_eq!(q.filters[0].field, "age");
    }

    #[test]
    #[ignore = "implement Query::limit"]
    fn test_limit_sets_value() {
        let q = Query::new().limit(5);
        assert_eq!(q.limit, Some(5));
    }

    #[test]
    #[ignore = "implement Query::offset"]
    fn test_offset_sets_value() {
        let q = Query::new().offset(10);
        assert_eq!(q.offset, 10);
    }

    // ── Query::matches ────────────────────────────────────────

    #[test]
    #[ignore = "implement Query::matches"]
    fn test_matches_eq_filter_true() {
        let doc = make_doc(1, "Alice", 30);
        let q = Query::new().filter("name", Operator::Eq, json!("Alice"));
        assert!(q.matches(&doc));
    }

    #[test]
    #[ignore = "implement Query::matches"]
    fn test_matches_eq_filter_false() {
        let doc = make_doc(1, "Bob", 25);
        let q = Query::new().filter("name", Operator::Eq, json!("Alice"));
        assert!(!q.matches(&doc));
    }

    #[test]
    #[ignore = "implement Query::matches"]
    fn test_matches_gt_filter() {
        let doc = make_doc(1, "Alice", 30);
        let q = Query::new().filter("age", Operator::Gt, json!(18));
        assert!(q.matches(&doc));
        let q2 = Query::new().filter("age", Operator::Gt, json!(30));
        assert!(!q2.matches(&doc)); // not strictly greater than 30
    }

    #[test]
    #[ignore = "implement Query::matches"]
    fn test_matches_multiple_filters_all_must_pass() {
        let doc = make_doc(1, "Alice", 30);
        let q = Query::new()
            .filter("age", Operator::Gte, json!(18))
            .filter("name", Operator::Eq, json!("Alice"));
        assert!(q.matches(&doc));

        let q2 = Query::new()
            .filter("age", Operator::Gte, json!(18))
            .filter("name", Operator::Eq, json!("Bob"));
        assert!(!q2.matches(&doc));
    }

    #[test]
    #[ignore = "implement Query::matches"]
    fn test_matches_missing_field_returns_false() {
        let doc = make_doc(1, "Alice", 30);
        let q = Query::new().filter("email", Operator::Eq, json!("x@y.com"));
        assert!(!q.matches(&doc));
    }

    #[test]
    #[ignore = "implement Query::matches"]
    fn test_matches_contains_operator() {
        let doc = make_doc(1, "Alice Wonderland", 30);
        let q = Query::new().filter("name", Operator::Contains, json!("Wonder"));
        assert!(q.matches(&doc));
    }
}
