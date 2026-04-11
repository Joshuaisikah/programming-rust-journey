// Ch10 — Enums & Pattern Matching | Ch06 — Expressions
//
// CONCEPTS:
//   Enum with data     — Command variants carry different payload types
//   match expression   — exhaustive pattern matching on Command
//   if let             — concise single-branch pattern matching
//   Result with ?      — error propagation through parse_command
//   Block expressions  — match arms can be multi-line blocks

use crate::errors::CliError;
use crate::models::{Priority, Task};

// ── Command enum ──────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub enum Command {
    /// Add a new task: name and priority
    Add { name: String, priority: Priority },
    /// List all tasks, optionally including completed ones
    List { show_done: bool },
    /// Mark a task complete by id
    Complete(u32),
    /// Remove a task by id
    Delete(u32),
    /// Search tasks by substring
    Search(String),
}

// ── Parsing ───────────────────────────────────────────────────

/// Parse CLI args (everything after the binary name) into a Command.
///
/// Expected formats:
///   ["add",      "<name>", "<priority>"]
///   ["list"]  or ["list", "--all"]
///   ["done",     "<id>"]
///   ["delete",   "<id>"]
///   ["search",   "<query>"]
pub fn parse_command(args: &[String]) -> Result<Command, CliError> {
    todo!("match args[0] and parse the remaining args into the right variant")
}

// ── Execution ─────────────────────────────────────────────────

/// Execute a Command against a mutable task list.
/// Returns a human-readable success message.
pub fn execute(cmd: Command, tasks: &mut Vec<Task>) -> Result<String, CliError> {
    todo!("match cmd and apply it to tasks; return Ok(message) or Err")
}

/// Generate the next available task id (1-based, always increasing).
fn next_id(tasks: &[Task]) -> u32 {
    todo!("return tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1")
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    // ── parse_command ─────────────────────────────────────────

    #[test]
    #[ignore = "implement parse_command"]
    fn test_parse_add() {
        let cmd = parse_command(&args(&["add", "buy milk", "high"])).unwrap();
        assert_eq!(
            cmd,
            Command::Add { name: "buy milk".into(), priority: Priority::High }
        );
    }

    #[test]
    #[ignore = "implement parse_command"]
    fn test_parse_list_default() {
        let cmd = parse_command(&args(&["list"])).unwrap();
        assert_eq!(cmd, Command::List { show_done: false });
    }

    #[test]
    #[ignore = "implement parse_command"]
    fn test_parse_list_all() {
        let cmd = parse_command(&args(&["list", "--all"])).unwrap();
        assert_eq!(cmd, Command::List { show_done: true });
    }

    #[test]
    #[ignore = "implement parse_command"]
    fn test_parse_done() {
        let cmd = parse_command(&args(&["done", "3"])).unwrap();
        assert_eq!(cmd, Command::Complete(3));
    }

    #[test]
    #[ignore = "implement parse_command"]
    fn test_parse_delete() {
        let cmd = parse_command(&args(&["delete", "7"])).unwrap();
        assert_eq!(cmd, Command::Delete(7));
    }

    #[test]
    #[ignore = "implement parse_command"]
    fn test_parse_search() {
        let cmd = parse_command(&args(&["search", "milk"])).unwrap();
        assert_eq!(cmd, Command::Search("milk".into()));
    }

    #[test]
    #[ignore = "implement parse_command"]
    fn test_parse_unknown_command_errors() {
        assert!(parse_command(&args(&["fly"])).is_err());
    }

    #[test]
    #[ignore = "implement parse_command"]
    fn test_parse_add_bad_priority_errors() {
        assert!(parse_command(&args(&["add", "task", "urgent"])).is_err());
    }

    #[test]
    #[ignore = "implement parse_command"]
    fn test_parse_done_non_numeric_id_errors() {
        assert!(parse_command(&args(&["done", "abc"])).is_err());
    }

    // ── execute ───────────────────────────────────────────────

    #[test]
    #[ignore = "implement execute"]
    fn test_execute_add_appends_task() {
        let mut tasks = vec![];
        execute(
            Command::Add { name: "buy milk".into(), priority: Priority::High },
            &mut tasks,
        ).unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].name, "buy milk");
        assert!(!tasks[0].done);
    }

    #[test]
    #[ignore = "implement execute"]
    fn test_execute_complete_marks_task_done() {
        let mut tasks = vec![Task::new(1, "write tests", Priority::High)];
        execute(Command::Complete(1), &mut tasks).unwrap();
        assert!(tasks[0].done);
    }

    #[test]
    #[ignore = "implement execute"]
    fn test_execute_complete_not_found_errors() {
        let mut tasks = vec![];
        let result = execute(Command::Complete(99), &mut tasks);
        assert!(result.is_err());
    }

    #[test]
    #[ignore = "implement execute"]
    fn test_execute_delete_removes_task() {
        let mut tasks = vec![
            Task::new(1, "first", Priority::Low),
            Task::new(2, "second", Priority::Low),
        ];
        execute(Command::Delete(1), &mut tasks).unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, 2);
    }

    #[test]
    #[ignore = "implement execute"]
    fn test_execute_search_filters_by_name() {
        let mut tasks = vec![
            Task::new(1, "buy milk", Priority::Low),
            Task::new(2, "write code", Priority::High),
        ];
        let result = execute(Command::Search("milk".into()), &mut tasks).unwrap();
        assert!(result.contains("buy milk"));
        assert!(!result.contains("write code"));
    }
}
