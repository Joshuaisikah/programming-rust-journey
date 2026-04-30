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
    if args.is_empty() {
        return Err(CliError::InvalidCommand("no command provided".into()));
    }
    match args[0].as_str() {
        "add" => {
            if args.len() < 3 {
                return Err(CliError::InvalidCommand("add requires name and priority".into()));
            }
            let name = args[1].clone();
            let priority = Priority::from_str(&args[2])?;
            Ok(Command::Add { name, priority })
        }
        "list" => {
            let show_done = args.len() > 1 && args[1] == "--all";
            Ok(Command::List { show_done })
        }
        "done" => {
            if args.len() < 2 {
                return Err(CliError::InvalidCommand("done requires an id".into()));
            }
            let id = args[1].parse::<u32>()
                .map_err(|_| CliError::InvalidId(args[1].clone()))?;
            Ok(Command::Complete(id))
        }
        "delete" => {
            if args.len() < 2 {
                return Err(CliError::InvalidCommand("delete requires an id".into()));
            }
            let id = args[1].parse::<u32>()
                .map_err(|_| CliError::InvalidId(args[1].clone()))?;
            Ok(Command::Delete(id))
        }
        "search" => {
            if args.len() < 2 {
                return Err(CliError::InvalidCommand("search requires a query".into()));
            }
            let query = args[1].clone();
            Ok(Command::Search(query))
        }
        _ => Err(CliError::InvalidCommand(format!("unknown subcommand '{}'", args[0]))),
    }
}

// ── Execution ─────────────────────────────────────────────────

/// Execute a Command against a mutable task list.
/// Returns a human-readable success message.
pub fn execute(cmd: Command, tasks: &mut Vec<Task>) -> Result<String, CliError> {
    match cmd {
        Command::Add { name, priority } => {
            let id = next_id(tasks);
            let task = Task::new(id, name.clone(), priority);
            tasks.push(task);
            Ok(format!("Added task {} with id {}", name, id))
        }
        Command::List { show_done } => {
            let filtered: Vec<&Task> = tasks
                .iter()
                .filter(|t| show_done || !t.is_done())
                .collect();
            if filtered.is_empty() {
                Ok("No tasks found".into())
            } else {
                Ok(format_task_list(&filtered))
            }
        }
        Command::Complete(id) => {
            let task = tasks
                .iter_mut()
                .find(|t| t.id == id)
                .ok_or_else(|| CliError::NotFound(format!("task with id {}", id)))?;
            task.complete();
            Ok(format!("Marked task {} as complete", id))
        }
        Command::Delete(id) => {
            let initial_len = tasks.len();
            tasks.retain(|t| t.id != id);
            if tasks.len() == initial_len {
                Err(CliError::NotFound(format!("task with id {}", id)))
            } else {
                Ok(format!("Deleted task {}", id))
            }
        }
        Command::Search(query) => {
            let results: Vec<&Task> = tasks
                .iter()
                .filter(|t| t.name.to_lowercase().contains(&query.to_lowercase()))
                .collect();
            if results.is_empty() {
                Ok(format!("No tasks found matching '{}'", query))
            } else {
                Ok(format_task_list(&results))
            }
        }
    }
}

fn format_task_list(tasks: &[&Task]) -> String {
    tasks
        .iter()
        .map(|t| {
            let status = if t.done { "Tick" } else { "pending " };
            format!("[{}] {}: {} ({})", status, t.id, t.name, t.priority.as_str())
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Generate the next available task id (1-based, always increasing).
fn next_id(tasks: &[Task]) -> u32 {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
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
    fn test_parse_add() {
        let cmd = parse_command(&args(&["add", "buy milk", "high"])).unwrap();
        assert_eq!(
            cmd,
            Command::Add { name: "buy milk".into(), priority: Priority::High }
        );
    }

    #[test]
    fn test_parse_list_default() {
        let cmd = parse_command(&args(&["list"])).unwrap();
        assert_eq!(cmd, Command::List { show_done: false });
    }

    #[test]
    fn test_parse_list_all() {
        let cmd = parse_command(&args(&["list", "--all"])).unwrap();
        assert_eq!(cmd, Command::List { show_done: true });
    }

    #[test]
    fn test_parse_done() {
        let cmd = parse_command(&args(&["done", "3"])).unwrap();
        assert_eq!(cmd, Command::Complete(3));
    }

    #[test]
    fn test_parse_delete() {
        let cmd = parse_command(&args(&["delete", "7"])).unwrap();
        assert_eq!(cmd, Command::Delete(7));
    }

    #[test]
    fn test_parse_search() {
        let cmd = parse_command(&args(&["search", "milk"])).unwrap();
        assert_eq!(cmd, Command::Search("milk".into()));
    }

    #[test]
    fn test_parse_unknown_command_errors() {
        assert!(parse_command(&args(&["fly"])).is_err());
    }

    #[test]
    fn test_parse_add_bad_priority_errors() {
        assert!(parse_command(&args(&["add", "task", "urgent"])).is_err());
    }

    #[test]
    fn test_parse_done_non_numeric_id_errors() {
        assert!(parse_command(&args(&["done", "abc"])).is_err());
    }

    // ── execute ───────────────────────────────────────────────

    #[test]
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
    fn test_execute_complete_marks_task_done() {
        let mut tasks = vec![Task::new(1, "write tests", Priority::High)];
        execute(Command::Complete(1), &mut tasks).unwrap();
        assert!(tasks[0].done);
    }

    #[test]
    fn test_execute_complete_not_found_errors() {
        let mut tasks = vec![];
        let result = execute(Command::Complete(99), &mut tasks);
        assert!(result.is_err());
    }

    #[test]
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
