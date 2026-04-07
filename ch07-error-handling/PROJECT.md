# Chapter 7 Project: HTTP Download Manager

Build a download manager with proper error handling.

## What You're Building

Download files with retries, timeouts, and error recovery.

```bash
cargo run download https://example.com/file.zip
cargo run batch urls.txt
```

## Features

1. **Download files** - Handle network errors
2. **Retry logic** - Recover from failures
3. **Progress tracking** - Show download status
4. **Custom errors** - Define your own error types
5. **? operator** - Propagate errors elegantly

## Concepts

```rust
// Result type everywhere
fn download(url: &str) -> Result<Vec<u8>, DownloadError> {
    let response = fetch(url)?;  // ? propagates errors
    let bytes = response.bytes()?;
    Ok(bytes)
}

// Custom error types
enum DownloadError {
    Network(String),
    Timeout,
    FileNotFound,
}
```

Master error handling! ⚠️
