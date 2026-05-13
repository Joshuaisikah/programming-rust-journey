// capstone-02 — JSON Document Database
//
// ── HOW IT ALL CONNECTS ───────────────────────────────────────
//
//   Database::open("./mydb")
//       ↓  creates the directory if missing; loads existing collections from disk
//   db.collection("users")
//       ↓  loads users.jsonl from disk → Collection, or creates a new empty one
//   collection.insert(fields)     → DocumentId  (auto-assigned, increments each insert)
//   collection.get(id)            → Option<&Document>
//   collection.update(id, patch)  → merges new fields into the existing Document
//   collection.delete(id)         → removes and returns the Document
//       ↓
//   collection.find(&query)       → Vec<&Document>
//       Query::new()
//           .filter("age", Operator::Gte, json!(18))  ← evaluates Query::matches() per doc
//           .sort_by("name", SortOrder::Asc)          ← sorts the matched results
//           .limit(10)                                 ← truncates to first N
//       ↓
//   storage::save(&collection, path)   → writes each Document as one JSON line (.jsonl)
//   storage::load(path)                → reads it back on next Database::open
//
//   Index lives inside Collection: field_name → Vec<DocumentId>
//   so find() can skip documents that don't match an indexed field.
//
//   Transaction (WAL): before writing to disk, append the operation to a log file
//   so a crash mid-write can be replayed and completed next time.
//
// ── IMPLEMENTATION ORDER ──────────────────────────────────────
//
//   1. document.rs     — Document::new, from_json, get, set, remove, to_json
//                        Every other module uses Document. Start here.
//
//   2. error.rs        — DbError: NotFound, DuplicateId, InvalidData, Io
//                        Add variants as you need them in later modules.
//
//   3. query.rs        — Query::new, .filter(), .sort_by(), .limit(), .offset()
//                        Then Query::matches(doc) — evaluate all filters.
//
//   4. index.rs        — Index::new, add(id, value), remove(id), lookup(value)
//                        Used inside Collection to speed up find().
//
//   5. collection.rs   — Collection::new, insert, insert_with_id, get, update,
//                        delete, find, count, create_index
//                        Core of the project. Depends on 1–4.
//
//   6. storage.rs      — save(collection, path), load(path) → Collection
//                        Each line in the .jsonl file = one Document::to_json().
//
//   7. transaction.rs  — Wal::new, log(op), apply(collection), recover(path)
//                        Wrap multi-step writes for crash safety. Do last.
//
//   8. lib.rs          — Database::open(path), db.collection(name)
//                        Wires storage load/save into the Database handle.
//
// ── ONCE COMPLETE ─────────────────────────────────────────────

use json_db::Database;

fn main() {
    println!("json-db — Capstone 2");
    println!();
    println!("Run `cargo test -p capstone-02-json-db` to track progress.");

    // Uncomment once implemented:
    //
    // use json_db::query::{Operator, Query, SortOrder};
    // use serde_json::json;
    //
    // // Open (or create) the database directory
    // let db = Database::open("./mydb").unwrap();
    //
    // // Get a collection (loaded from disk or empty)
    // let mut users = db.collection("users").unwrap();
    //
    // // Insert documents
    // let alice_id = users.insert({
    //     let mut f = std::collections::HashMap::new();
    //     f.insert("name".into(), json!("Alice"));
    //     f.insert("age".into(),  json!(30));
    //     f
    // }).unwrap();
    //
    // users.insert({
    //     let mut f = std::collections::HashMap::new();
    //     f.insert("name".into(), json!("Bob"));
    //     f.insert("age".into(),  json!(17));
    //     f
    // }).unwrap();
    //
    // println!("Total documents: {}", users.count()); // → 2
    //
    // // Query: adults only, sorted by name
    // let q = Query::new()
    //     .filter("age", Operator::Gte, json!(18))
    //     .sort_by("name", SortOrder::Asc)
    //     .limit(10);
    // let adults = users.find(&q);
    // println!("Adults: {}", adults.len()); // → 1 (Alice)
    //
    // // Update a field
    // let mut patch = std::collections::HashMap::new();
    // patch.insert("age".into(), json!(31));
    // users.update(alice_id, patch).unwrap();
    // println!("Alice age: {:?}", users.get(alice_id).unwrap().get("age")); // → 31
    //
    // // Delete
    // users.delete(alice_id).unwrap();
    // println!("After delete: {}", users.count()); // → 1
}
