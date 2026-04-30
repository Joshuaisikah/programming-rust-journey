# Capstone 02 — JSON Document Database

## What You're Building

An embedded document database — a simplified MongoDB that runs entirely inside
your program without any external server. You store JSON documents, query them,
and persist them to disk as `.jsonl` files.

```rust
let db = Database::open("./mydata")?;
let users = db.collection("users")?;

users.insert(json!({ "name": "Joshua", "age": 27 }))?;

let results = users.query()
    .filter("age", GreaterThan(25))
    .sort_by("name")
    .limit(10)
    .run()?;
```

## The Real Point

This project teaches you what happens *inside* a database — the data structures
and algorithms that make storage and retrieval fast and correct:

- **Document model** — every document has a unique ID and arbitrary JSON fields.
  `serde` serializes and deserializes Rust structs to/from JSON automatically.

- **Indexes** — a raw scan of every document for every query is too slow.
  You build an in-memory index (a `HashMap`) so lookups by field are fast.
  This is exactly how real databases work internally.

- **Persistence** — data must survive the program restarting. You serialize
  collections to `.jsonl` (one JSON document per line) and reload them on startup.

- **Transactions** — what happens if the program crashes mid-write?
  You implement a write-ahead log: write the intent first, then apply it.
  If you crash, replay the log on startup. This is how every serious database
  handles durability.

## Architecture: How the Files Connect

```
main.rs  (demo/CLI driver)
  │
  └── lib.rs exposes:  Database → Collection → Document
                                      │
        ┌─────────────────────────────┼──────────────────────┐
        │                             │                      │
   storage.rs                    index.rs               transaction.rs
   (reads/writes .jsonl files)   (in-memory HashMap     (write-ahead log,
                                  for fast lookup)       crash recovery)
        │
   document.rs         query.rs           error.rs
   (Document type,     (query builder,    (DbError, used
    DocumentId, serde   filter/sort/limit  by every module)
    derive)             logic)
```

**`collection.rs`** sits at the center — it is the object you interact with.
It calls `storage` to persist, `index` to find fast, and `transaction` to be safe.
It uses `document` as its data type and `query` to interpret what the user asked for.

## Why Each File Exists

**`main.rs`** — A demo driver or simple CLI. Shows how to open a database, insert
documents, and run queries. Kept thin; all real logic lives in the library.

**`lib.rs`** — The public API. Re-exports `Database`, `Collection`, `Document`,
`Query`, `DbError` so callers have one clean import path.

**`document.rs`** — Defines what a document *is*: a unique ID and a JSON value.
Separated so the `Document` type can be used across `collection`, `storage`,
`index`, and `query` without creating import cycles.

**`collection.rs`** — The main object users interact with. Coordinates the other
modules: calls `storage` to load/save, `index` to look up fast, `transaction` to
log writes, `query` to filter results. It is the orchestrator, not the worker.

**`query.rs`** — The query builder lives here: `.filter()`, `.sort_by()`,
`.limit()`. Separated from `collection.rs` so query logic can grow independently
and be tested without creating a real database on disk.

**`storage.rs`** — All disk I/O. Reads and writes `.jsonl` files, serializes and
deserializes documents with `serde_json`. Separated so the rest of the codebase
never touches file handles — swap the storage format and nothing else changes.

**`index.rs`** — The in-memory index: a `HashMap<FieldValue, Vec<DocumentId>>`.
Separated so the indexing strategy can change (B-tree, inverted index) without
touching collection logic. Also isolated for testing: you can test index correctness
without touching disk.

**`transaction.rs`** — Write-ahead log implementation. Every mutation is written
to a log file before it is applied. If the process crashes, the log is replayed
on next startup to restore consistency. Separated because this is a safety
mechanism — it must be correct in isolation before being trusted.

**`error.rs`** — All error variants for the database (`NotFound`, `InvalidQuery`,
`IoError`, `ParseError`). Separated so every module can return `DbError` without
importing anything else from the crate.

## Chapters It Combines

Ch09 Structs · Ch10 Enums & Patterns · Ch11 Traits & Generics ·
Ch16 Collections · Ch18 I/O

## Mental Model

You are building a mini SQLite — but for JSON instead of tables. After this
capstone you will understand why databases like MongoDB, SQLite, and RocksDB are
designed the way they are: not because someone was clever, but because those
designs are the direct solution to the problems of storage, speed, and crash safety.
