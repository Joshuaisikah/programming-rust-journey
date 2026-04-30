// Ch06 — Expressions (format strings as expressions)
// Ch17 — Strings (format!, padding, alignment)

use crate::models::{Priority, Task};

pub fn print_usage() {
    println!("Usage:");
    println!("  cargo run -- add <name> <priority>   # priority: high / medium / low");
    println!("  cargo run -- list [--all]");
    println!("  cargo run -- done <id>");
    println!("  cargo run -- delete <id>");
    println!("  cargo run -- search <query>");
}

/// Format a priority for display.
pub fn format_priority(p: &Priority) -> &'static str {
    match p {
        Priority::High   => "[HIGH]",
        Priority::Medium => "[MED] ",
        Priority::Low    => "[LOW] ",
    }
}

/// Print a single task as one formatted line.
pub fn print_task(task: &Task) {
    let status = if task.is_done() { "✓" } else { " " };
    println!("{:>3}  [{}]  {}  {}", task.id, status, format_priority(&task.priority), task.name);
}

/// Print a table of tasks with a header row.
pub fn print_task_list(tasks: &[Task]) {
    println!("{:>3}  {:5}  {:6}  {}", "ID", "DONE", "PRI", "NAME");
    println!("{}", "-".repeat(40));
    for task in tasks {
        print_task(task);
    }
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_priority_high() {
        assert_eq!(format_priority(&Priority::High), "[HIGH]");
    }

    #[test]
    fn test_format_priority_medium() {
        assert_eq!(format_priority(&Priority::Medium), "[MED] ");
    }

    #[test]
    fn test_format_priority_low() {
        assert_eq!(format_priority(&Priority::Low), "[LOW] ");
    }
}
