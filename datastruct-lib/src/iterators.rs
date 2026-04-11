// Ch15 — Iterators
//
// CONCEPTS:
//   Iterator trait     — implement next() → Option<Self::Item>
//   IntoIterator       — allow for-loop on your type
//   Adapter methods    — map, filter, take, skip, enumerate, zip, chain
//   Lazy evaluation    — iterators produce values on demand
//   collect()          — consume an iterator into a collection
//
// IMPLEMENTING Iterator requires only one method:
//   fn next(&mut self) -> Option<Self::Item>
// Everything else (map, filter, take, fold, etc.) is given for free.

// ── Fibonacci ─────────────────────────────────────────────────

/// Infinite iterator producing 0, 1, 1, 2, 3, 5, 8, 13, ...
pub struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        todo!("Fibonacci {{ a: 0, b: 1 }}")
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        todo!("yield self.a, then advance: a = b, b = a + b")
    }
}

// ── StepBy ────────────────────────────────────────────────────

/// Iterator that counts from `start` by `step` indefinitely.
/// StepBy::new(0, 3) produces 0, 3, 6, 9, ...
pub struct StepBy {
    current: i64,
    step: i64,
}

impl StepBy {
    pub fn new(start: i64, step: i64) -> Self {
        todo!("StepBy {{ current: start, step }}")
    }
}

impl Iterator for StepBy {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        todo!("yield current, then advance by step")
    }
}

// ── RunningSum ────────────────────────────────────────────────

/// Wraps any Iterator<Item = i64> and yields cumulative sums.
/// Input [1, 2, 3, 4] → Output [1, 3, 6, 10]
pub struct RunningSum<I: Iterator<Item = i64>> {
    inner: I,
    sum: i64,
}

impl<I: Iterator<Item = i64>> RunningSum<I> {
    pub fn new(inner: I) -> Self {
        todo!("RunningSum {{ inner, sum: 0 }}")
    }
}

impl<I: Iterator<Item = i64>> Iterator for RunningSum<I> {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        todo!("get next from inner, add to self.sum, return Some(self.sum)")
    }
}

// ── Chunks ────────────────────────────────────────────────────

/// Iterator that yields non-overlapping fixed-size chunks from a Vec.
/// Chunks::new(vec![1,2,3,4,5], 2) → [1,2], [3,4], [5]
pub struct Chunks<T> {
    data: Vec<T>,
    size: usize,
    pos: usize,
}

impl<T: Clone> Chunks<T> {
    pub fn new(data: Vec<T>, size: usize) -> Self {
        todo!("Chunks {{ data, size, pos: 0 }}")
    }
}

impl<T: Clone> Iterator for Chunks<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        todo!("slice data[pos..pos+size], advance pos by size, return None when exhausted")
    }
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Fibonacci ─────────────────────────────────────────────

    #[test]
    #[ignore = "implement Fibonacci"]
    fn test_fibonacci_first_eight() {
        let result: Vec<u64> = Fibonacci::new().take(8).collect();
        assert_eq!(result, vec![0, 1, 1, 2, 3, 5, 8, 13]);
    }

    #[test]
    #[ignore = "implement Fibonacci"]
    fn test_fibonacci_is_infinite() {
        // Should not panic — take() prevents infinite loop
        let _: Vec<u64> = Fibonacci::new().take(50).collect();
    }

    #[test]
    #[ignore = "implement Fibonacci"]
    fn test_fibonacci_adapter_chain() {
        // First Fibonacci number > 100
        let result = Fibonacci::new().find(|&x| x > 100);
        assert_eq!(result, Some(144));
    }

    // ── StepBy ────────────────────────────────────────────────

    #[test]
    #[ignore = "implement StepBy"]
    fn test_stepby_threes() {
        let result: Vec<i64> = StepBy::new(0, 3).take(5).collect();
        assert_eq!(result, vec![0, 3, 6, 9, 12]);
    }

    #[test]
    #[ignore = "implement StepBy"]
    fn test_stepby_negative_step() {
        let result: Vec<i64> = StepBy::new(10, -2).take(4).collect();
        assert_eq!(result, vec![10, 8, 6, 4]);
    }

    #[test]
    #[ignore = "implement StepBy"]
    fn test_stepby_step_one_is_normal_count() {
        let result: Vec<i64> = StepBy::new(5, 1).take(3).collect();
        assert_eq!(result, vec![5, 6, 7]);
    }

    // ── RunningSum ────────────────────────────────────────────

    #[test]
    #[ignore = "implement RunningSum"]
    fn test_running_sum_basic() {
        let result: Vec<i64> = RunningSum::new(vec![1, 2, 3, 4].into_iter()).collect();
        assert_eq!(result, vec![1, 3, 6, 10]);
    }

    #[test]
    #[ignore = "implement RunningSum"]
    fn test_running_sum_empty() {
        let result: Vec<i64> = RunningSum::new(vec![].into_iter()).collect();
        assert!(result.is_empty());
    }

    #[test]
    #[ignore = "implement RunningSum"]
    fn test_running_sum_single_element() {
        let result: Vec<i64> = RunningSum::new(vec![7].into_iter()).collect();
        assert_eq!(result, vec![7]);
    }

    // ── Chunks ────────────────────────────────────────────────

    #[test]
    #[ignore = "implement Chunks"]
    fn test_chunks_even_split() {
        let result: Vec<Vec<i32>> = Chunks::new(vec![1, 2, 3, 4], 2).collect();
        assert_eq!(result, vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    #[ignore = "implement Chunks"]
    fn test_chunks_remainder() {
        let result: Vec<Vec<i32>> = Chunks::new(vec![1, 2, 3, 4, 5], 2).collect();
        assert_eq!(result, vec![vec![1, 2], vec![3, 4], vec![5]]);
    }

    #[test]
    #[ignore = "implement Chunks"]
    fn test_chunks_size_larger_than_data() {
        let result: Vec<Vec<i32>> = Chunks::new(vec![1, 2], 10).collect();
        assert_eq!(result, vec![vec![1, 2]]);
    }
}
