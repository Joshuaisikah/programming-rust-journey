// ============================================================
// moves.rs — Move Semantics
// ============================================================
//
// KEY CONCEPTS:
//   - When you assign a non-Copy value, ownership MOVES — the
//     original variable becomes invalid.
//   - Passing a value to a function moves it (unless it's Copy).
//   - Functions can return ownership back to the caller.
//   - Moves are zero-cost: no data is copied, just ownership transfers.
//
// OWNERSHIP RULES:
//   1. Each value has exactly one owner.
//   2. Only one owner at a time.
//   3. When the owner goes out of scope, the value is dropped.

use crate::display::section;

/// Demonstrates move semantics with Strings and structs.
pub fn demo_moves() {
    section("MOVE SEMANTICS");

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);

    fn takes_ownership(s: String) {
        println!("got: {}", s);
    }
    let s = String::from("world");
    takes_ownership(s);

    fn gives_back(s: String) -> String {
        s
    }
    let s1 = String::from("returned");
    let s2 = gives_back(s1);
    println!("{}", s2);

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = p1;
    println!("({}, {})", p2.x, p2.y);
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    /// A move transfers ownership; the moved-from binding is gone.
    /// We verify this by moving into a function and getting back.
    #[test]
    fn test_move_into_function_and_return() {
        fn round_trip(s: String) -> String {
            s
        }
        let original = String::from("hello");
        let returned = round_trip(original);
        assert_eq!(returned, "hello");
        // `original` is no longer valid here — compile error if used
    }

    #[test]
    fn test_move_via_assignment() {
        let s1 = String::from("move me");
        let s2 = s1; // s1 is moved
        assert_eq!(s2, "move me");
    }

    #[test]
    fn test_struct_move() {
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }
        let p1 = Point { x: 1, y: 2 };
        let p2 = p1; // p1 moved
        assert_eq!(p2, Point { x: 1, y: 2 });
    }

    #[test]
    fn test_vec_move() {
        let v1 = vec![1, 2, 3];
        let v2 = v1; // v1 moved
        assert_eq!(v2, vec![1, 2, 3]);
    }
}
