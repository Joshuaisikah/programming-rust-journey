// walker.rs — Directory traversal with .gitignore support
//
// Uses the `ignore` crate (same engine as ripgrep) to:
//   - Walk directories recursively
//   - Respect .gitignore and .ignore files
//   - Optionally follow symlinks
//
// Returns an iterator of file paths to search.

use std::path::PathBuf;
use ignore::WalkBuilder;

/// Build a file iterator for the given root paths.
///
/// Respects .gitignore by default.
/// Set `follow_links` to also follow symbolic links.
pub fn build_walker(
    paths: &[PathBuf],
    follow_links: bool,
) -> impl Iterator<Item = PathBuf> {
    // Placeholder so the compiler can infer the return type.
    // Replace with WalkBuilder + filter when implementing.
    let _ = (paths, follow_links);
    todo!("use WalkBuilder, chain all paths, filter to files only, yield PathBufs");
    #[allow(unreachable_code)]
    std::iter::empty::<PathBuf>()
}

/// Collect all searchable file paths under the given roots.
/// Convenience wrapper around `build_walker` for tests.
pub fn collect_paths(paths: &[PathBuf], follow_links: bool) -> Vec<PathBuf> {
    todo!("build_walker(...).collect()")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn tmp_dir() -> tempfile::TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    #[ignore = "implement collect_paths"]
    fn test_single_file_path_returns_that_file() {
        let dir = tmp_dir();
        let file = dir.path().join("hello.rs");
        fs::write(&file, "fn main() {}").unwrap();

        let paths = collect_paths(&[file.clone()], false);
        assert_eq!(paths, vec![file]);
    }

    #[test]
    #[ignore = "implement collect_paths"]
    fn test_directory_walks_recursively() {
        let dir = tmp_dir();
        fs::write(dir.path().join("a.rs"), "a").unwrap();
        let sub = dir.path().join("sub");
        fs::create_dir(&sub).unwrap();
        fs::write(sub.join("b.rs"), "b").unwrap();

        let mut paths = collect_paths(&[dir.path().to_path_buf()], false);
        paths.sort();
        assert_eq!(paths.len(), 2);
    }

    #[test]
    #[ignore = "implement collect_paths"]
    fn test_gitignore_excludes_ignored_files() {
        let dir = tmp_dir();
        fs::write(dir.path().join(".gitignore"), "*.log\n").unwrap();
        fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();
        fs::write(dir.path().join("debug.log"), "ignored").unwrap();

        let paths = collect_paths(&[dir.path().to_path_buf()], false);
        assert!(paths.iter().all(|p| p.extension().and_then(|e| e.to_str()) != Some("log")));
        assert_eq!(paths.len(), 1); // only main.rs
    }
}
