// Ch14 — Closures
//
// CONCEPTS:
//   Fn / FnMut / FnOnce  — closure trait hierarchy
//     Fn      — can be called repeatedly, borrows env immutably
//     FnMut   — can be called repeatedly, borrows env mutably
//     FnOnce  — can be called only once, consumes captured env
//   Higher-order functions — functions that accept or return closures
//   Move closures          — `move |x| ...` captures by value
//   impl Fn return type    — return a closure from a function

// ── Higher-order functions ────────────────────────────────────

/// Apply `f` to each element of `v`, returning a new Vec.
/// Equivalent to v.into_iter().map(f).collect() — implement manually.
pub fn map_vec<T, U, F: Fn(T) -> U>(v: Vec<T>, f: F) -> Vec<U> {
    todo!("iterate v, call f on each element, collect into Vec<U>")
}

/// Keep only elements of `v` for which `predicate` returns true.
pub fn filter_vec<T, F: Fn(&T) -> bool>(v: Vec<T>, predicate: F) -> Vec<T> {
    todo!("iterate v, retain elements where predicate(&elem) is true")
}

/// Reduce `v` to a single value by applying `f` left-to-right.
/// Returns None for an empty slice.
pub fn fold_vec<T: Clone, F: Fn(T, T) -> T>(v: &[T], f: F) -> Option<T> {
    todo!("start with v[0].clone(), fold the rest with f")
}

// ── Function composition ──────────────────────────────────────

/// Return a new function that applies `f` then `g`.
/// compose(f, g)(x) == g(f(x))
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| todo!("call f then g — use move to capture both")
}

/// Apply `f` exactly `n` times: apply_n(f, 3)(x) == f(f(f(x)))
pub fn apply_n<T: Clone, F: Fn(T) -> T>(f: F, n: u32) -> impl Fn(T) -> T {
    move |x| todo!("loop n times applying f, or use recursion")
}

// ── Closure capture demos ─────────────────────────────────────

/// Return a closure that adds `delta` to its argument (captures by value).
pub fn make_adder(delta: i32) -> impl Fn(i32) -> i32 {
    let _ = delta;
    move |_x| todo!("x + delta")
}

/// Return a counter closure that increments on each call (FnMut capture).
pub fn make_counter() -> impl FnMut() -> i32 {
    move || todo!("capture a mutable count, increment and return it")
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "implement map_vec"]
    fn test_map_vec_doubles() {
        let result = map_vec(vec![1, 2, 3], |x| x * 2);
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    #[ignore = "implement map_vec"]
    fn test_map_vec_to_strings() {
        let result = map_vec(vec![1, 2, 3], |x| x.to_string());
        assert_eq!(result, vec!["1", "2", "3"]);
    }

    #[test]
    #[ignore = "implement filter_vec"]
    fn test_filter_vec_evens() {
        let result = filter_vec(vec![1, 2, 3, 4, 5, 6], |x| x % 2 == 0);
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    #[ignore = "implement filter_vec"]
    fn test_filter_vec_empty_result() {
        let result = filter_vec(vec![1, 3, 5], |x| x % 2 == 0);
        assert!(result.is_empty());
    }

    #[test]
    #[ignore = "implement fold_vec"]
    fn test_fold_vec_sum() {
        assert_eq!(fold_vec(&[1, 2, 3, 4], |a, b| a + b), Some(10));
    }

    #[test]
    #[ignore = "implement fold_vec"]
    fn test_fold_vec_empty_returns_none() {
        assert_eq!(fold_vec::<i32, _>(&[], |a, b| a + b), None);
    }

    #[test]
    #[ignore = "implement compose"]
    fn test_compose_add_then_double() {
        let add_one = |x: i32| x + 1;
        let double  = |x: i32| x * 2;
        let f = compose(add_one, double);
        assert_eq!(f(3), 8); // (3 + 1) * 2
    }

    #[test]
    #[ignore = "implement apply_n"]
    fn test_apply_n_three_times() {
        let add_one = |x: i32| x + 1;
        let add_three = apply_n(add_one, 3);
        assert_eq!(add_three(10), 13);
    }

    #[test]
    #[ignore = "implement apply_n"]
    fn test_apply_n_zero_is_identity() {
        let double = |x: i32| x * 2;
        let noop = apply_n(double, 0);
        assert_eq!(noop(5), 5);
    }

    #[test]
    #[ignore = "implement make_adder"]
    fn test_make_adder() {
        let add5 = make_adder(5);
        assert_eq!(add5(10), 15);
        assert_eq!(add5(0),   5);
    }

    #[test]
    #[ignore = "implement make_counter"]
    fn test_make_counter_increments() {
        let mut count = make_counter();
        assert_eq!(count(), 1);
        assert_eq!(count(), 2);
        assert_eq!(count(), 3);
    }
}
