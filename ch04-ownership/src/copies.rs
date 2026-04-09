// ============================================================
// copies.rs — Copy Types vs Clone
// ============================================================
//
// KEY CONCEPTS:
//   - Types that implement `Copy` are duplicated on assignment —
//     the original stays valid. Examples: i32, f64, bool, char,
//     tuples/arrays of Copy types.
//   - Types that implement `Clone` can be explicitly duplicated
//     with .clone(). This may allocate (e.g. String).
//   - You can derive both: #[derive(Copy, Clone)] — but only if
//     ALL fields are Copy.
//   - Box<T>, Vec<T>, String are NOT Copy (heap ownership).

use crate::display::section;

/// Demonstrates Copy types and explicit Clone.
pub fn demo_copies() {
    section("COPY TYPES vs CLONE");

    // TODO: demo 1 — Copy types are implicitly duplicated
    //   let x: i32 = 42;
    //   let y = x;          // x is COPIED, not moved
    //   println!("{} {}", x, y);  // both valid

    // TODO: demo 2 — bool, char, f64 are also Copy
    //   let flag = true;
    //   let also_flag = flag;
    //   println!("{} {}", flag, also_flag);

    // TODO: demo 3 — tuple of Copy types is Copy
    //   let pair = (1_i32, 2.0_f64);
    //   let also_pair = pair;
    //   println!("{:?} {:?}", pair, also_pair);

    // TODO: demo 4 — String is NOT Copy; use .clone()
    //   let s1 = String::from("original");
    //   let s2 = s1.clone();   // explicit heap copy
    //   println!("{} {}", s1, s2);

    // TODO: demo 5 — derive Copy + Clone on a struct
    //   #[derive(Copy, Clone, Debug)]
    //   struct Meters(f64);
    //   let a = Meters(1.5);
    //   let b = a;   // copied
    //   println!("{:?} {:?}", a, b);

    println!("(implement the copy/clone demos above)");
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    #[test]
    fn test_i32_is_copy() {
        let x: i32 = 10;
        let y = x;
        assert_eq!(x, 10); // x still valid — it was copied
        assert_eq!(y, 10);
    }

    #[test]
    fn test_bool_is_copy() {
        let a = true;
        let b = a;
        assert!(a);
        assert!(b);
    }

    #[test]
    fn test_string_clone() {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        assert_eq!(s1, s2);
        assert_ne!(s1.as_ptr(), s2.as_ptr()); // different heap allocations
    }

    #[test]
    fn test_derived_copy_struct() {
        #[derive(Copy, Clone, PartialEq, Debug)]
        struct Meters(f64);

        let a = Meters(3.14);
        let b = a; // copy
        assert_eq!(a, b);
    }

    // TODO: add tests for tuple copy, array copy, Vec clone, etc.

    #[test]
    fn placeholder() {
        assert!(true);
    }
}
