// ============================================================
// borrows.rs — Shared and Mutable References
// ============================================================
//
// KEY CONCEPTS:
//
//   Shared reference  &T
//     Read-only access. Many can exist at the same time.
//     The owner retains ownership; the reference is just a borrow.
//
//   Mutable reference  &mut T
//     Read-write access. Only ONE can exist at a time.
//     No shared references may exist while a mutable one is alive.
//
//   Reborrow
//     You can borrow from a reference: &&T or &mut (&mut T).
//
//   Borrow checker enforces: no use-after-free, no data races.

use crate::display::section;

/// Demonstrates shared and mutable borrowing.
pub fn demo_borrows() {
    section("SHARED & MUTABLE REFERENCES");

    // TODO: demo 1 — shared reference (read-only borrow)
    //   let s = String::from("hello");
    //   let r1 = &s;
    //   let r2 = &s;   // multiple shared refs OK
    //   println!("{} {}", r1, r2);
    //   // s is still valid here

    // TODO: demo 2 — mutable reference
    //   let mut s = String::from("hello");
    //   let r = &mut s;
    //   r.push_str(", world");
    //   println!("{}", r);
    //   // cannot use s here while r is alive

    // TODO: demo 3 — reference as function parameter
    //   fn length(s: &String) -> usize { s.len() }
    //   let s = String::from("hello");
    //   println!("len = {}", length(&s));
    //   println!("{}", s);  // s still valid — not moved

    // TODO: demo 4 — mutable reference in function
    //   fn append(s: &mut String) { s.push_str("!"); }
    //   let mut s = String::from("hello");
    //   append(&mut s);
    //   println!("{}", s);

    println!("(implement the borrow demos above)");
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    #[test]
    fn test_shared_ref_does_not_move() {
        let s = String::from("hello");
        let r = &s;
        assert_eq!(r.len(), 5);
        assert_eq!(s.len(), 5); // s still valid
    }

    #[test]
    fn test_multiple_shared_refs() {
        let x = 42_i32;
        let r1 = &x;
        let r2 = &x;
        assert_eq!(*r1, *r2);
    }

    #[test]
    fn test_mutable_ref_modifies_value() {
        let mut v = vec![1, 2, 3];
        let r = &mut v;
        r.push(4);
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_function_borrows_without_move() {
        fn first_char(s: &str) -> char {
            s.chars().next().unwrap()
        }
        let s = String::from("rust");
        let c = first_char(&s);
        assert_eq!(c, 'r');
        assert_eq!(s, "rust"); // still owned
    }

    #[test]
    fn test_mutable_borrow_in_function() {
        fn double(n: &mut i32) { *n *= 2; }
        let mut x = 5;
        double(&mut x);
        assert_eq!(x, 10);
    }

    // TODO: add tests for nested borrows, reborrow, etc.

    #[test]
    fn placeholder() {
        assert!(true);
    }
}
