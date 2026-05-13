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
    let mut result = Vec::with_capacity(v.len());
    for item in v {
        result.push(f(item));
    }
    result
}

/// Keep only elements of `v` for which `predicate` returns true.
pub fn filter_vec<T, F: Fn(&T) -> bool>(v: Vec<T>, predicate: F) -> Vec<T> {
  let mut result = Vec::new();
    for item in v  {
         if predicate(&item) {
             result.push(item);
         }
    }
    result

}

/// Reduce `v` to a single value by applying `f` left-to-right.
/// Returns None for an empty slice.
pub fn fold_vec<T: Clone, F: Fn(T, T) -> T>(v: &[T], f: F) -> Option<T> {
    if v.is_empty() {
        return None;
    }
    let mut result = v[0].clone();
    for item in &v[1..] {
        result = f(result, item.clone());
    }
    Some(result)
}

// ── Function composition ──────────────────────────────────────

/// Return a new function that applies `f` then `g`.
/// compose(f, g)(x) == g(f(x))
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x|g(f(x))

}

/// Apply `f` exactly `n` times: apply_n(f, 3)(x) == f(f(f(x)))
pub fn apply_n<T: Clone, F: Fn(T) -> T>(f: F, n: u32) -> impl Fn(T) -> T {
    move |mut x|{
        for _ in 0..n {
            x=f(x.clone());
        }
        x
    }
}

// ── Closure capture demos ─────────────────────────────────────

/// Return a closure that adds `delta` to its argument (captures by value).
pub fn make_adder(delta: i32) -> impl Fn(i32) -> i32 {
    let _ = delta;
    move |_x| _x +delta
}

/// Return a counter closure that increments on each call (FnMut capture).
pub fn make_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

// ── Demo ──────────────────────────────────────────────────────

pub fn demo() {
    println!("=== Closures / Functional demo ===");
    let doubled = map_vec(vec![1, 2, 3, 4], |x| x * 2);
    println!("map_vec [1,2,3,4] ×2        : {:?}", doubled);
    let evens = filter_vec(vec![1, 2, 3, 4, 5, 6], |x| x % 2 == 0);
    println!("filter_vec evens            : {:?}", evens);
    let sum = fold_vec(&[1, 2, 3, 4, 5], |a, b| a + b);
    println!("fold_vec sum                : {:?}", sum);
    let add_then_double = compose(|x: i32| x + 1, |x| x * 2);
    println!("compose(+1, ×2)(3)          : {}", add_then_double(3));
    let add_three = apply_n(|x: i32| x + 1, 3);
    println!("apply_n(+1, 3)(10)          : {}", add_three(10));
    let add5 = make_adder(5);
    println!("make_adder(5)(10)           : {}", add5(10));
    let mut counter = make_counter();
    println!("make_counter ×3             : {}, {}, {}", counter(), counter(), counter());
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_vec_doubles() {
        let result = map_vec(vec![1, 2, 3], |x| x * 2);
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_map_vec_to_strings() {
        let result = map_vec(vec![1, 2, 3], |x| x.to_string());
        assert_eq!(result, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_filter_vec_evens() {
        let result = filter_vec(vec![1, 2, 3, 4, 5, 6], |x| x % 2 == 0);
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_filter_vec_empty_result() {
        let result = filter_vec(vec![1, 3, 5], |x| x % 2 == 0);
        assert!(result.is_empty());
    }

    #[test]
    fn test_fold_vec_sum() {
        assert_eq!(fold_vec(&[1, 2, 3, 4], |a, b| a + b), Some(10));
    }

    #[test]
    fn test_fold_vec_empty_returns_none() {
        assert_eq!(fold_vec::<i32, _>(&[], |a, b| a + b), None);
    }

    #[test]
    fn test_compose_add_then_double() {
        let add_one = |x: i32| x + 1;
        let double  = |x: i32| x * 2;
        let f = compose(add_one, double);
        assert_eq!(f(3), 8); // (3 + 1) * 2
    }

    #[test]
    fn test_apply_n_three_times() {
        let add_one = |x: i32| x + 1;
        let add_three = apply_n(add_one, 3);
        assert_eq!(add_three(10), 13);
    }

    #[test]
    fn test_apply_n_zero_is_identity() {
        let double = |x: i32| x * 2;
        let noop = apply_n(double, 0);
        assert_eq!(noop(5), 5);
    }

    #[test]
    fn test_make_adder() {
        let add5 = make_adder(5);
        assert_eq!(add5(10), 15);
        assert_eq!(add5(0),   5);
    }

    #[test]
    fn test_make_counter_increments() {
        let mut count = make_counter();
        assert_eq!(count(), 1);
        assert_eq!(count(), 2);
        assert_eq!(count(), 3);
    }
}
