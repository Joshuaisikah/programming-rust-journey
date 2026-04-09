// ============================================================
// lifetimes.rs — Lifetime Annotations
// ============================================================
//
// KEY CONCEPTS:
//
//   Every reference has a lifetime — the scope it is valid in.
//   The compiler infers lifetimes in most cases (elision rules).
//
//   ELISION RULES (when you don't need to write 'a):
//     1. Each reference parameter gets its own lifetime.
//     2. If there's exactly one input lifetime, it's assigned to outputs.
//     3. If one parameter is &self / &mut self, its lifetime is
//        assigned to all outputs.
//
//   When elision doesn't apply, write 'a explicitly:
//     fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }
//
//   'static — the reference lives for the entire program.
//   String literals are &'static str.

use crate::display::section;

/// Returns the longer of two string slices.
/// Both inputs and the output share lifetime 'a — the shorter of the two.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

/// A struct that holds a reference — requires a lifetime annotation.
pub struct Important<'a> {
    pub text: &'a str,
}

impl<'a> Important<'a> {
    pub fn new(text: &'a str) -> Self {
        Important { text }
    }

    /// Elision applies here: output lifetime = &self lifetime.
    pub fn content(&self) -> &str {
        self.text
    }
}

/// Demonstrates lifetime annotations and elision.
pub fn demo_lifetimes() {
    section("LIFETIME ANNOTATIONS");

    // TODO: demo 1 — longest() requires explicit 'a
    //   let s1 = String::from("long string");
    //   let result;
    //   {
    //       let s2 = String::from("xy");
    //       result = longest(s1.as_str(), s2.as_str());
    //       println!("longest: {}", result);
    //   }

    // TODO: demo 2 — struct holding a reference
    //   let novel = String::from("Call me Ishmael. Some years ago...");
    //   let first_sentence = novel.split('.').next().unwrap();
    //   let ann = Important::new(first_sentence);
    //   println!("important: {}", ann.content());

    // TODO: demo 3 — 'static lifetime
    //   let s: &'static str = "I live for the whole program";
    //   println!("{}", s);

    println!("(implement the lifetime demos above)");
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_first_wins_on_tie() {
        assert_eq!(longest("abc", "xyz"), "abc");
    }

    #[test]
    fn test_longest_returns_longer() {
        assert_eq!(longest("hello world", "hi"), "hello world");
        assert_eq!(longest("hi", "hello world"), "hello world");
    }

    #[test]
    fn test_struct_with_lifetime() {
        let text = String::from("important text");
        let ann = Important::new(&text);
        assert_eq!(ann.content(), "important text");
    }

    #[test]
    fn test_static_str() {
        let s: &'static str = "always valid";
        assert_eq!(s.len(), 12);
    }

    // TODO: add tests for nested lifetimes, lifetime in impl blocks, etc.

    #[test]
    fn placeholder() {
        assert!(true);
    }
}
