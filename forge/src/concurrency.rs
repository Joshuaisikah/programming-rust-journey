// Ch19 — Concurrency
//
// CONCEPTS:
//   thread::spawn       — spawn an OS thread; returns JoinHandle<T>
//   move closure        — transfer ownership of captured vars into the thread
//   Arc<T>              — atomically ref-counted shared pointer (Send + Sync)
//   Mutex<T>            — mutual exclusion; lock() returns MutexGuard<T>
//   mpsc::channel       — multi-producer single-consumer message passing
//   JoinHandle::join    — block until thread finishes, return its result

/// Sum all values in `numbers` by splitting the work across two threads.
/// parallel_sum(vec![1,2,3,4]) → 10
pub fn parallel_sum(numbers: Vec<i64>) -> i64 {
    todo!()
}

/// Apply `f` to every element of `items` using one thread per element.
/// Results are returned in the same order as the input.
pub fn concurrent_map<T, U>(items: Vec<T>, f: impl Fn(T) -> U + Send + Sync + 'static) -> Vec<U>
where
    T: Send + 'static,
    U: Send + 'static,
{
    todo!()
}

/// Send all `values` through an mpsc channel and collect them on the receiving end.
/// Returns them in the order they were sent.
pub fn roundtrip_channel(values: Vec<i32>) -> Vec<i32> {
    todo!()
}

/// Increment a shared counter from `n` threads and return the final count.
pub fn shared_counter(n: usize) -> i32 {
    todo!()
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "implement parallel_sum"]
    fn test_parallel_sum_basic() {
        assert_eq!(parallel_sum(vec![1, 2, 3, 4]), 10);
    }

    #[test]
    #[ignore = "implement parallel_sum"]
    fn test_parallel_sum_empty() {
        assert_eq!(parallel_sum(vec![]), 0);
    }

    #[test]
    #[ignore = "implement parallel_sum"]
    fn test_parallel_sum_single() {
        assert_eq!(parallel_sum(vec![42]), 42);
    }

    #[test]
    #[ignore = "implement concurrent_map"]
    fn test_concurrent_map_doubles() {
        let result = concurrent_map(vec![1_i32, 2, 3, 4], |x| x * 2);
        assert_eq!(result, vec![2, 4, 6, 8]);
    }

    #[test]
    #[ignore = "implement concurrent_map"]
    fn test_concurrent_map_preserves_order() {
        let result = concurrent_map(vec![3_i32, 1, 4, 1, 5], |x| x + 10);
        assert_eq!(result, vec![13, 11, 14, 11, 15]);
    }

    #[test]
    #[ignore = "implement roundtrip_channel"]
    fn test_roundtrip_channel_basic() {
        let result = roundtrip_channel(vec![10, 20, 30]);
        assert_eq!(result, vec![10, 20, 30]);
    }

    #[test]
    #[ignore = "implement roundtrip_channel"]
    fn test_roundtrip_channel_empty() {
        assert!(roundtrip_channel(vec![]).is_empty());
    }

    #[test]
    #[ignore = "implement shared_counter"]
    fn test_shared_counter_four_threads() {
        assert_eq!(shared_counter(4), 4);
    }

    #[test]
    #[ignore = "implement shared_counter"]
    fn test_shared_counter_zero_threads() {
        assert_eq!(shared_counter(0), 0);
    }
}
