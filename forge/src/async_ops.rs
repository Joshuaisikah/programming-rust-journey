// Ch20 — Asynchronous Programming
//
// CONCEPTS:
//   async fn            — returns a Future; body doesn't run until .awaited
//   .await              — suspend until the Future resolves
//   tokio::spawn        — schedule a Future on the Tokio thread pool
//   tokio::join!        — drive multiple futures concurrently, wait for all
//   tokio::select!      — wait for the first of several futures to complete
//   JoinHandle          — handle to a spawned task; .await on it to get result

/// Return `value` after sleeping for `millis` milliseconds.
/// Demonstrates async fn + tokio::time::sleep.
pub async fn delayed<T: Send + 'static>(value: T, millis: u64) -> T {
    todo!()
}

/// Run all handles concurrently and collect their results in order.
/// Demonstrates awaiting a Vec of JoinHandles.
pub async fn join_all(handles: Vec<tokio::task::JoinHandle<i32>>) -> Vec<i32> {
    todo!()
}

/// Return the sum of `a` and `b`, computed concurrently with tokio::join!.
/// Both async blocks should run at the same time.
pub async fn concurrent_add(a: i32, b: i32) -> i32 {
    todo!()
}

/// Spawn `n` tasks, each returning its index, and collect all results.
pub async fn spawn_n(n: usize) -> Vec<usize> {
    todo!()
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "implement delayed"]
    async fn test_delayed_returns_value() {
        let result = delayed(42_i32, 0).await;
        assert_eq!(result, 42);
    }

    #[tokio::test]
    #[ignore = "implement delayed"]
    async fn test_delayed_string() {
        let result = delayed("hello", 0).await;
        assert_eq!(result, "hello");
    }

    #[tokio::test]
    #[ignore = "implement join_all"]
    async fn test_join_all_collects_in_order() {
        let handles = vec![
            tokio::spawn(async { 10_i32 }),
            tokio::spawn(async { 20_i32 }),
            tokio::spawn(async { 30_i32 }),
        ];
        assert_eq!(join_all(handles).await, vec![10, 20, 30]);
    }

    #[tokio::test]
    #[ignore = "implement join_all"]
    async fn test_join_all_empty() {
        assert!(join_all(vec![]).await.is_empty());
    }

    #[tokio::test]
    #[ignore = "implement concurrent_add"]
    async fn test_concurrent_add() {
        assert_eq!(concurrent_add(3, 4).await, 7);
        assert_eq!(concurrent_add(0, 0).await, 0);
    }

    #[tokio::test]
    #[ignore = "implement spawn_n"]
    async fn test_spawn_n_returns_indices() {
        let result = spawn_n(5).await;
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[tokio::test]
    #[ignore = "implement spawn_n"]
    async fn test_spawn_n_zero() {
        assert!(spawn_n(0).await.is_empty());
    }
}
