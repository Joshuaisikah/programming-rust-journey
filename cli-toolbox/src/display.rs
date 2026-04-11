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
    todo!("return a display string, e.g. \"[HIGH]\", \"[MED]\", \"[LOW]\"")
}

/// Print a single task as one formatted line.
pub fn print_task(task: &Task) {
    todo!("print: id, done marker, priority, name — aligned")
}

/// Print a table of tasks with a header row.
pub fn print_task_list(tasks: &[Task]) {
    todo!("print header, then call print_task for each task")
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "implement format_priority"]
    fn test_format_priority_high() {
        assert_eq!(format_priority(&Priority::High), "[HIGH]");
    }

    #[test]
    #[ignore = "implement format_priority"]
    fn test_format_priority_medium() {
        assert_eq!(format_priority(&Priority::Medium), "[MED]");
    }

    #[test]
    #[ignore = "implement format_priority"]
    fn test_format_priority_low() {
        assert_eq!(format_priority(&Priority::Low), "[LOW]");
    }
}
