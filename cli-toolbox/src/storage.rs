// Ch18 — I/O
//
// CONCEPTS:
//   std::fs::read_to_string   — read an entire file into a String
//   std::fs::write            — write bytes/string to a file atomically
//   serde_json::from_str      — deserialize JSON text into a Rust value
//   serde_json::to_string_pretty — serialize a Rust value to formatted JSON
//   ? operator                — propagate I/O and parse errors up to the caller
//
// HOW IT WORKS:
//   The task list is persisted as a JSON array in a file on disk.
//
//   load_tasks:
//     1. Check whether the file exists.
//        - If it doesn't, there are no saved tasks yet → return an empty Vec.
//     2. Read the whole file into a String with fs::read_to_string.
//        - This can fail (permissions, disk error) → the ? propagates CliError::Io.
//     3. Parse the JSON string into Vec<Task> with serde_json::from_str.
//        - This can fail (corrupt file, schema change) → ? propagates CliError::Json.
//     4. Return the Vec.
//
//   save_tasks:
//     1. Serialize Vec<Task> to a pretty-printed JSON String.
//     2. Write that String to the file with fs::write, replacing whatever was there.
//        - fs::write creates the file if it doesn't exist, overwrites if it does.
//     3. Return Ok(()) on success, or propagate the error.
//
// WHY pretty JSON?
//   serde_json::to_string_pretty produces human-readable output (one field per
//   line, indented). You can open tasks.json in any text editor or pipe it through
//   `jq` to inspect your data — handy while learning.

use std::fs;
use std::path::Path;

use crate::errors::CliError;
use crate::models::Task;

/// Path to the JSON file used for persistence.
pub const TASKS_FILE: &str = "tasks.json";

/// Load tasks from disk. Returns an empty list if the file doesn't exist yet.
pub fn load_tasks() -> Result<Vec<Task>, CliError> {
    if !Path::new(TASKS_FILE).exists() {
        return Ok(vec![]);
    }
    let contents = fs::read_to_string(TASKS_FILE)?;             // ? → CliError::Io
    let tasks = serde_json::from_str(&contents)
        .map_err(|e| CliError::Json(e.to_string()))?;           // ? → CliError::Json
    Ok(tasks)
}

/// Save tasks to disk, overwriting any previous file.
pub fn save_tasks(tasks: &[Task]) -> Result<(), CliError> {
    let json = serde_json::to_string_pretty(tasks)
        .map_err(|e| CliError::Json(e.to_string()))?;
    fs::write(TASKS_FILE, json)?;                               // ? → CliError::Io
    Ok(())
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Priority;

    // Serialize a list of tasks to JSON and deserialize it back.
    // We don't touch the real TASKS_FILE here — pure in-memory round-trip.
    #[test]
    fn test_round_trip_serialize() {
        let tasks = vec![
            Task::new(1, "buy milk", Priority::High),
            Task::new(2, "write code", Priority::Low),
        ];

        // Serialize → JSON string
        let json = serde_json::to_string_pretty(&tasks).unwrap();
        assert!(json.contains("buy milk"));
        assert!(json.contains("High"));

        // Deserialize back
        let loaded: Vec<Task> = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].name, "buy milk");
        assert_eq!(loaded[1].priority, Priority::Low);
    }

    #[test]
    fn test_round_trip_preserves_done_flag() {
        let mut task = Task::new(1, "finish chapter", Priority::Medium);
        task.complete();

        let json = serde_json::to_string(&vec![task]).unwrap();
        let loaded: Vec<Task> = serde_json::from_str(&json).unwrap();
        assert!(loaded[0].is_done());
    }
}
