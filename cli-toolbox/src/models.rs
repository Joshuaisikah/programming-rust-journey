// Ch09 — Structs | Ch10 — Enums & Patterns
//
// CONCEPTS:
//   Named-field struct  — Task holds id, name, priority, done
//   impl blocks         — methods live separately from the data definition
//   Enum as type        — Priority is a closed set of variants
//   Derived traits      — Debug, Clone, PartialEq via #[derive]
//   Pattern matching    — used inside Priority::from_str

use crate::errors::CliError;

// ── Priority ──────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Priority {
    /// Parse a case-insensitive string into a Priority variant.
    /// Returns Err(CliError::InvalidPriority) for unknown strings.
    pub fn from_str(s: &str) -> Result<Self, CliError> {
        todo!("match s.to_lowercase().as_str() → High/Medium/Low or Err")
    }

    /// Return the canonical lowercase name of this variant.
    pub fn as_str(&self) -> &'static str {
        todo!("match self → \"high\" / \"medium\" / \"low\"")
    }
}

// ── Task ──────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub priority: Priority,
    pub done: bool,
}

impl Task {
    /// Create a new incomplete task.
    pub fn new(id: u32, name: impl Into<String>, priority: Priority) -> Self {
        todo!("construct Task with done: false")
    }

    /// Mark this task as complete.
    pub fn complete(&mut self) {
        todo!("set self.done = true")
    }

    /// Return true if this task has been completed.
    pub fn is_done(&self) -> bool {
        todo!("return self.done")
    }
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Priority tests ────────────────────────────────────────

    #[test]
    #[ignore = "implement Priority::from_str"]
    fn test_priority_from_str_high() {
        assert_eq!(Priority::from_str("high").unwrap(), Priority::High);
        assert_eq!(Priority::from_str("HIGH").unwrap(), Priority::High);
    }

    #[test]
    #[ignore = "implement Priority::from_str"]
    fn test_priority_from_str_all_variants() {
        assert_eq!(Priority::from_str("medium").unwrap(), Priority::Medium);
        assert_eq!(Priority::from_str("low").unwrap(), Priority::Low);
    }

    #[test]
    #[ignore = "implement Priority::from_str"]
    fn test_priority_from_str_invalid() {
        assert!(Priority::from_str("urgent").is_err());
        assert!(Priority::from_str("").is_err());
    }

    #[test]
    #[ignore = "implement Priority::as_str"]
    fn test_priority_as_str() {
        assert_eq!(Priority::High.as_str(), "high");
        assert_eq!(Priority::Medium.as_str(), "medium");
        assert_eq!(Priority::Low.as_str(), "low");
    }

    #[test]
    fn test_priority_ordering() {
        // High < Medium < Low (High is most urgent)
        assert!(Priority::High < Priority::Medium);
        assert!(Priority::Medium < Priority::Low);
    }

    // ── Task tests ────────────────────────────────────────────

    #[test]
    #[ignore = "implement Task::new"]
    fn test_task_new_starts_incomplete() {
        let t = Task::new(1, "buy milk", Priority::Medium);
        assert_eq!(t.id, 1);
        assert_eq!(t.name, "buy milk");
        assert_eq!(t.priority, Priority::Medium);
        assert!(!t.done);
    }

    #[test]
    #[ignore = "implement Task::complete and Task::is_done"]
    fn test_task_complete() {
        let mut t = Task::new(2, "write tests", Priority::High);
        assert!(!t.is_done());
        t.complete();
        assert!(t.is_done());
    }

    #[test]
    #[ignore = "implement Task::new"]
    fn test_task_clone_is_independent() {
        let t1 = Task::new(3, "original", Priority::Low);
        let mut t2 = t1.clone();
        t2.complete();
        assert!(!t1.done); // t1 unaffected by t2's mutation
    }
}
