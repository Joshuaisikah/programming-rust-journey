# Capstone 2: JSON Document Database (Like MongoDB Lite)

Build a real database that stores JSON documents with queries, indexes, and persistence.

**Chapters Combined:** 9 (Structs), 10 (Enums/Patterns), 11 (Traits/Generics), 16 (Collections), 18 (I/O)

**🔥 THIS IS A KILLER PROJECT - PORTFOLIO SHOWPIECE!**

---

## 🎯 What You're Building

A production-ready NoSQL document database:
- **Store** JSON documents in collections
- **Query** with filters (find, where, sort)
- **Index** fields for fast lookups
- **Persist** to disk with transactions
- **ACID** guarantees (Atomicity, Consistency, Isolation, Durability)
- **CLI + Library** - usable as both

**Use cases:** Cache, config storage, embedded database, learning SQL alternatives

---

## 📋 Architecture

```
jsondb/
├── src/
│   ├── lib.rs           // Public API
│   ├── database.rs      // Database manager
│   ├── collection.rs    // Collection operations
│   ├── document.rs      // Document type
│   ├── query.rs         // Query builder
│   ├── index.rs         // Indexing system
│   ├── storage.rs       // Persistence layer
│   ├── transaction.rs   // ACID transactions
│   └── cli.rs           // CLI interface
├── tests/
│   └── integration_tests.rs
└── benches/
    └── benchmarks.rs
```

---

## 🔨 Core Types

### Document

**Convention:** Use serde_json for JSON

```rust
// document.rs
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    id: String,                    // Unique ID
    data: HashMap<String, Value>,  // JSON data
    metadata: Metadata,
}

#[derive(Debug, Clone, PartialEq)]
struct Metadata {
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    version: u64,
}

impl Document {
    pub fn new(data: HashMap<String, Value>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            data,
            metadata: Metadata {
                created_at: Utc::now(),
                updated_at: Utc::now(),
                version: 1,
            },
        }
    }

    pub fn get(&self, field: &str) -> Option<&Value> {
        self.data.get(field)
    }

    pub fn set(&mut self, field: String, value: Value) {
        self.data.insert(field, value);
        self.metadata.updated_at = Utc::now();
        self.metadata.version += 1;
    }

    pub fn matches(&self, query: &Query) -> bool {
        query.evaluate(self)
    }
}
```

---

### Collection

**Convention:** Use generics for flexibility

```rust
// collection.rs
use std::collections::HashMap;

pub struct Collection {
    name: String,
    documents: HashMap<String, Document>,
    indexes: HashMap<String, Index>,
}

impl Collection {
    pub fn new(name: String) -> Self {
        Self {
            name,
            documents: HashMap::new(),
            indexes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, doc: Document) -> Result<String> {
        let id = doc.id.clone();

        // Update indexes
        for (field, index) in &mut self.indexes {
            if let Some(value) = doc.get(field) {
                index.add(&id, value);
            }
        }

        self.documents.insert(id.clone(), doc);
        Ok(id)
    }

    pub fn find_one(&self, query: &Query) -> Option<&Document> {
        // Try to use index first (optimization!)
        if let Some(field) = query.indexed_field() {
            if let Some(index) = self.indexes.get(field) {
                return index.lookup(query).and_then(|id| self.documents.get(id));
            }
        }

        // Fall back to full scan
        self.documents.values().find(|doc| doc.matches(query))
    }

    pub fn find(&self, query: &Query) -> Vec<&Document> {
        self.documents
            .values()
            .filter(|doc| doc.matches(query))
            .collect()
    }

    pub fn update(&mut self, query: &Query, updates: HashMap<String, Value>) -> Result<usize> {
        let mut count = 0;
        let ids: Vec<String> = self.find(query).iter().map(|d| d.id.clone()).collect();

        for id in ids {
            if let Some(doc) = self.documents.get_mut(&id) {
                for (field, value) in &updates {
                    doc.set(field.clone(), value.clone());
                }
                count += 1;
            }
        }

        Ok(count)
    }

    pub fn delete(&mut self, query: &Query) -> Result<usize> {
        let ids: Vec<String> = self.find(query).iter().map(|d| d.id.clone()).collect();
        let count = ids.len();

        for id in ids {
            self.documents.remove(&id);
        }

        Ok(count)
    }

    pub fn create_index(&mut self, field: String) {
        let mut index = Index::new(field.clone());

        // Build index from existing documents
        for (id, doc) in &self.documents {
            if let Some(value) = doc.get(&field) {
                index.add(id, value);
            }
        }

        self.indexes.insert(field, index);
    }
}
```

---

### Query Builder

**Convention:** Fluent API for ergonomics

```rust
// query.rs
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Query {
    filters: Vec<Filter>,
    sort: Option<Sort>,
    limit: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum Filter {
    Eq { field: String, value: Value },
    Gt { field: String, value: Value },
    Lt { field: String, value: Value },
    Contains { field: String, value: String },
    And(Vec<Filter>),
    Or(Vec<Filter>),
}

impl Query {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
            sort: None,
            limit: None,
        }
    }

    // Fluent API!
    pub fn eq(mut self, field: impl Into<String>, value: Value) -> Self {
        self.filters.push(Filter::Eq {
            field: field.into(),
            value,
        });
        self
    }

    pub fn gt(mut self, field: impl Into<String>, value: Value) -> Self {
        self.filters.push(Filter::Gt {
            field: field.into(),
            value,
        });
        self
    }

    pub fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }

    pub fn evaluate(&self, doc: &Document) -> bool {
        self.filters.iter().all(|filter| match filter {
            Filter::Eq { field, value } => {
                doc.get(field).map(|v| v == value).unwrap_or(false)
            }
            Filter::Gt { field, value } => {
                doc.get(field).map(|v| v > value).unwrap_or(false)
            }
            Filter::Lt { field, value } => {
                doc.get(field).map(|v| v < value).unwrap_or(false)
            }
            Filter::Contains { field, value } => doc
                .get(field)
                .and_then(|v| v.as_str())
                .map(|s| s.contains(value))
                .unwrap_or(false),
            Filter::And(filters) => filters.iter().all(|f| {
                let q = Query {
                    filters: vec![f.clone()],
                    sort: None,
                    limit: None,
                };
                q.evaluate(doc)
            }),
            Filter::Or(filters) => filters.iter().any(|f| {
                let q = Query {
                    filters: vec![f.clone()],
                    sort: None,
                    limit: None,
                };
                q.evaluate(doc)
            }),
        })
    }
}

// Usage example:
// let query = Query::new()
//     .eq("status", json!("active"))
//     .gt("age", json!(18))
//     .limit(10);
```

---

### Index

**Convention:** Use BTreeMap for sorted indexes

```rust
// index.rs
use std::collections::BTreeMap;

pub struct Index {
    field: String,
    map: BTreeMap<Value, Vec<String>>,  // value -> document IDs
}

impl Index {
    pub fn new(field: String) -> Self {
        Self {
            field,
            map: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, doc_id: &str, value: &Value) {
        self.map
            .entry(value.clone())
            .or_insert_with(Vec::new)
            .push(doc_id.to_string());
    }

    pub fn lookup(&self, query: &Query) -> Option<String> {
        // Extract value from query if it's an Eq filter on this field
        // Return first matching doc ID
        // This is a simplified version - real impl would be more sophisticated
        None
    }

    pub fn range(&self, start: &Value, end: &Value) -> Vec<String> {
        self.map
            .range(start..=end)
            .flat_map(|(_, ids)| ids.iter().cloned())
            .collect()
    }
}
```

---

### Persistence

**Convention:** Use Write-Ahead Log (WAL) for durability

```rust
// storage.rs
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

pub struct Storage {
    data_file: File,
    wal_file: File,  // Write-Ahead Log
}

impl Storage {
    pub fn open(path: &Path) -> Result<Self> {
        let data_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path.join("data.jsonl"))?;

        let wal_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path.join("wal.log"))?;

        Ok(Self { data_file, wal_file })
    }

    pub fn write_wal(&mut self, operation: &Operation) -> Result<()> {
        // Write to WAL first (durability!)
        let json = serde_json::to_string(operation)?;
        writeln!(self.wal_file, "{}", json)?;
        self.wal_file.sync_all()?;  // Force to disk!
        Ok(())
    }

    pub fn checkpoint(&mut self, collections: &HashMap<String, Collection>) -> Result<()> {
        // Write all data to data file
        // Clear WAL
        // This is called periodically
        Ok(())
    }

    pub fn load(&self) -> Result<HashMap<String, Collection>> {
        // 1. Load from data file
        // 2. Replay WAL for any uncommitted changes
        // 3. Return collections
        Ok(HashMap::new())
    }
}

#[derive(Serialize, Deserialize)]
enum Operation {
    Insert { collection: String, doc: Document },
    Update { collection: String, id: String, updates: HashMap<String, Value> },
    Delete { collection: String, id: String },
}
```

---

### Database

**Convention:** Single entry point, owns all collections

```rust
// database.rs
pub struct Database {
    collections: HashMap<String, Collection>,
    storage: Storage,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let storage = Storage::open(path.as_ref())?;
        let collections = storage.load()?;

        Ok(Self { collections, storage })
    }

    pub fn collection(&mut self, name: &str) -> &mut Collection {
        self.collections
            .entry(name.to_string())
            .or_insert_with(|| Collection::new(name.to_string()))
    }

    pub fn transaction(&mut self) -> Transaction {
        Transaction::new(self)
    }
}

// Usage:
// let mut db = Database::open("./data")?;
// let users = db.collection("users");
// users.insert(doc)?;
```

---

## 🎨 Public API

```rust
// lib.rs - Clean public API

pub use database::Database;
pub use document::Document;
pub use query::Query;
pub use collection::Collection;

// Fluent API example:
pub fn example() -> Result<()> {
    let mut db = Database::open("./mydb")?;

    // Insert
    let user = json!({
        "name": "Alice",
        "age": 30,
        "email": "alice@example.com"
    });
    db.collection("users").insert(Document::from_json(user))?;

    // Query
    let adults = db.collection("users")
        .find(&Query::new().gt("age", json!(18)))?;

    // Update
    db.collection("users")
        .update(
            &Query::new().eq("name", json!("Alice")),
            hashmap!{ "age" => json!(31) }
        )?;

    // Delete
    db.collection("users")
        .delete(&Query::new().eq("name", json!("Alice")))?;

    Ok(())
}
```

---

## 🧪 Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_insert_and_find() {
        let dir = TempDir::new().unwrap();
        let mut db = Database::open(dir.path()).unwrap();

        let doc = json!({ "name": "test", "value": 42 });
        db.collection("test").insert(Document::from_json(doc)).unwrap();

        let results = db.collection("test")
            .find(&Query::new().eq("name", json!("test")));

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_index_performance() {
        // Insert 10,000 documents
        // Query without index - measure time
        // Create index
        // Query with index - should be 10x+ faster
    }

    #[test]
    fn test_persistence() {
        let dir = TempDir::new().unwrap();

        {
            let mut db = Database::open(dir.path()).unwrap();
            db.collection("test").insert(Document::from_json(json!({"x": 1}))).unwrap();
        }

        // Reopen database
        let db = Database::open(dir.path()).unwrap();
        assert_eq!(db.collection("test").count(), 1);
    }
}
```

---

## 📦 Dependencies

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = "0.4"
uuid = { version = "1.0", features = ["v4"] }
thiserror = "2.0"
anyhow = "1.0"
clap = { version = "4.5", features = ["derive"] }
```

---

## ✅ Completion Checklist

### Core Features (MVP)
- [ ] Insert documents
- [ ] Find by query
- [ ] Update documents
- [ ] Delete documents
- [ ] Persist to disk
- [ ] Load from disk

### Advanced Features
- [ ] Indexes for fast lookups
- [ ] Write-Ahead Log (WAL)
- [ ] Transactions
- [ ] Query builder (fluent API)
- [ ] Collections

### Performance
- [ ] Handle 100,000+ documents
- [ ] Index queries < 10ms
- [ ] Benchmark vs HashMap

### Production Ready
- [ ] Error handling
- [ ] Tests (unit + integration)
- [ ] Documentation
- [ ] CLI interface

---

## 🚀 CLI Interface

```bash
# Start interactive shell
cargo run

jsondb> use mydb
jsondb> db.users.insert({ "name": "Alice", "age": 30 })
jsondb> db.users.find({ "age": { "$gt": 25 } })
jsondb> db.users.createIndex("age")
jsondb> db.users.count()
```

---

## 💡 Extension Ideas

### Level 1
- [ ] JSON schema validation
- [ ] Unique constraints
- [ ] Auto-increment IDs
- [ ] Backup/restore

### Level 2
- [ ] Aggregation pipeline
- [ ] Full-text search
- [ ] Replication
- [ ] HTTP REST API

### Level 3
- [ ] Sharding
- [ ] MVCC (Multi-Version Concurrency Control)
- [ ] Query optimizer
- [ ] Distributed consensus (Raft)

---

## 📚 Key Takeaways

✅ **Data structures** - HashMap, BTreeMap, when to use each
✅ **Serialization** - serde_json for persistence
✅ **Indexing** - Performance optimization
✅ **Transactions** - ACID properties
✅ **API design** - Fluent interfaces, builder pattern
✅ **File I/O** - Persistence, WAL, durability

---

**This is a REAL database. Put it on your portfolio!** 🔥💼
