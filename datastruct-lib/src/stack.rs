// Ch11 — Traits & Generics | Ch13 — Utility Traits | Ch15 — Iterators
//
// CONCEPTS:
//   Generic struct     — Stack<T> works for any type T
//   impl<T>            — implement methods on a generic type
//   Iterator trait     — IntoIterator and a borrowing StackIter
//   Default trait      — Stack::default() = empty stack
//   Display trait      — human-readable output
//   Drop trait         — cleanup on scope exit (Vec handles it here,
//                        but a raw-memory version would need explicit Drop)

use std::fmt;

// ── Stack<T> ──────────────────────────────────────────────────

pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    /// Create an empty stack.
    pub fn new() -> Self {
        todo!("Stack {{ data: vec![] }}")
    }

    /// Push a value onto the top of the stack.
    pub fn push(&mut self, item: T) {
        todo!("self.data.push(item)")
    }

    /// Remove and return the top value, or None if empty.
    pub fn pop(&mut self) -> Option<T> {
        todo!("self.data.pop()")
    }

    /// Return a reference to the top value without removing it.
    pub fn peek(&self) -> Option<&T> {
        todo!("self.data.last()")
    }

    /// Return true if the stack has no elements.
    pub fn is_empty(&self) -> bool {
        todo!("self.data.is_empty()")
    }

    /// Return the number of elements in the stack.
    pub fn len(&self) -> usize {
        todo!("self.data.len()")
    }
}

// ── Default ───────────────────────────────────────────────────

impl<T> Default for Stack<T> {
    fn default() -> Self {
        todo!("Stack::new()")
    }
}

// ── Display ───────────────────────────────────────────────────

impl<T: fmt::Display> fmt::Display for Stack<T> {
    /// Print from bottom to top: Stack[1, 2, 3↑]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("write elements separated by \", \", mark top with ↑")
    }
}

// ── Iterator (borrowing) ──────────────────────────────────────

pub struct StackIter<'a, T> {
    stack: &'a Stack<T>,
    index: usize, // iterates bottom → top
}

impl<'a, T> Iterator for StackIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        todo!("return stack.data[index] and advance index")
    }
}

impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = &'a T;
    type IntoIter = StackIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        todo!("StackIter {{ stack: self, index: 0 }}")
    }
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "implement Stack::new / is_empty"]
    fn test_new_stack_is_empty() {
        let s: Stack<i32> = Stack::new();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    #[ignore = "implement Stack::push / len"]
    fn test_push_increases_len() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        assert_eq!(s.len(), 2);
        assert!(!s.is_empty());
    }

    #[test]
    #[ignore = "implement Stack::pop"]
    fn test_pop_returns_lifo_order() {
        let mut s = Stack::new();
        s.push(10);
        s.push(20);
        s.push(30);
        assert_eq!(s.pop(), Some(30));
        assert_eq!(s.pop(), Some(20));
        assert_eq!(s.pop(), Some(10));
        assert_eq!(s.pop(), None);
    }

    #[test]
    #[ignore = "implement Stack::peek"]
    fn test_peek_does_not_remove() {
        let mut s = Stack::new();
        s.push(42);
        assert_eq!(s.peek(), Some(&42));
        assert_eq!(s.len(), 1); // still there
    }

    #[test]
    #[ignore = "implement Stack::peek"]
    fn test_peek_empty_returns_none() {
        let s: Stack<i32> = Stack::new();
        assert_eq!(s.peek(), None);
    }

    #[test]
    #[ignore = "implement IntoIterator for &Stack"]
    fn test_iterate_bottom_to_top() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        let collected: Vec<&i32> = s.into_iter().collect();
        assert_eq!(collected, vec![&1, &2, &3]);
    }

    #[test]
    #[ignore = "implement Default for Stack"]
    fn test_default_is_empty() {
        let s: Stack<String> = Stack::default();
        assert!(s.is_empty());
    }
}
