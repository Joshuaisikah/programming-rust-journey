// storage.rs — Persist a collection to disk as .jsonl (JSON Lines)
//
// Each document is one JSON object per line.
// Loading reads all lines and reconstructs the Collection.

use std::path::{Path, PathBuf};
use crate::collection::Collection;
use crate::error::DbError;

/// The file extension used for stored collections.
pub const FILE_EXT: &str = "jsonl";

/// Return the path where a collection is stored.
pub fn collection_path(db_dir: &Path, name: &str) -> PathBuf {
    todo!("db_dir.join(format!(\"{{name}}.{{FILE_EXT}}\"))")
}

/// Write a collection to disk. Creates or overwrites the file.
pub fn save(collection: &Collection, db_dir: &Path) -> Result<(), DbError> {
    todo!("serialize each document as one JSON line, write to collection_path")
}

/// Load a collection from disk. Returns an empty collection if the file does not exist.
pub fn load(name: &str, db_dir: &Path) -> Result<Collection, DbError> {
    todo!("read lines, parse each as Document, insert_with_id into a new Collection")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;
    use crate::document::Document;

    fn tmp_dir() -> tempfile::TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    #[ignore = "implement save / load"]
    fn test_save_and_load_round_trip() {
        let dir = tmp_dir();
        let mut c = Collection::new("items");
        let mut fields = HashMap::new();
        fields.insert("label".to_string(), json!("first"));
        c.insert(fields).unwrap();

        save(&c, dir.path()).unwrap();

        let loaded = load("items", dir.path()).unwrap();
        assert_eq!(loaded.count(), 1);
    }

    #[test]
    #[ignore = "implement load"]
    fn test_load_nonexistent_returns_empty_collection() {
        let dir = tmp_dir();
        let c = load("ghost", dir.path()).unwrap();
        assert_eq!(c.count(), 0);
        assert_eq!(c.name, "ghost");
    }

    #[test]
    #[ignore = "implement save / load"]
    fn test_saved_file_is_valid_jsonl() {
        let dir = tmp_dir();
        let mut c = Collection::new("things");
        let mut f = HashMap::new();
        f.insert("x".to_string(), json!(1));
        c.insert(f).unwrap();

        save(&c, dir.path()).unwrap();

        let path = collection_path(dir.path(), "things");
        let content = std::fs::read_to_string(path).unwrap();
        // Every non-empty line should be valid JSON
        for line in content.lines().filter(|l| !l.is_empty()) {
            assert!(serde_json::from_str::<serde_json::Value>(line).is_ok());
        }
    }
}
