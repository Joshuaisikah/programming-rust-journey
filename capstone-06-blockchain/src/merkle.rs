// merkle.rs — Merkle tree for transaction hashing
//
// A Merkle tree hashes a list of transaction IDs into a single root hash.
// The root is stored in each block header to commit to its transactions
// without storing all data in the header.
//
//         root
//        /    \
//      h01    h23
//     /   \  /   \
//    h0  h1 h2  h3
//
// If there's an odd number of leaves, duplicate the last leaf.

use sha2::{Digest, Sha256};

/// Compute the Merkle root hash of a list of data items.
/// Each item is hashed first, then combined pairwise up the tree.
/// Returns an empty hash string if `items` is empty.
pub fn merkle_root(items: &[impl AsRef<[u8]>]) -> String {
    todo!("hash each item, then combine pairwise until one hash remains")
}

/// SHA-256 hash of the input bytes, returned as a hex string.
pub fn sha256_hex(data: &[u8]) -> String {
    todo!("Sha256::digest(data), format as hex")
}

/// Combine two hash strings into a parent hash: sha256(left + right).
pub fn combine_hashes(left: &str, right: &str) -> String {
    todo!("sha256_hex((left + right).as_bytes())")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── sha256_hex ────────────────────────────────────────────

    #[test]
    #[ignore = "implement sha256_hex"]
    fn test_sha256_empty_input() {
        // SHA-256 of empty bytes is a known constant
        let h = sha256_hex(&[]);
        assert_eq!(h.len(), 64); // 32 bytes → 64 hex chars
        assert_eq!(h, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }

    #[test]
    #[ignore = "implement sha256_hex"]
    fn test_sha256_deterministic() {
        let h1 = sha256_hex(b"hello");
        let h2 = sha256_hex(b"hello");
        assert_eq!(h1, h2);
    }

    #[test]
    #[ignore = "implement sha256_hex"]
    fn test_sha256_different_inputs_differ() {
        assert_ne!(sha256_hex(b"hello"), sha256_hex(b"world"));
    }

    // ── merkle_root ───────────────────────────────────────────

    #[test]
    #[ignore = "implement merkle_root"]
    fn test_merkle_root_empty_is_empty_hash() {
        let empty: Vec<String> = vec![];
        let root = merkle_root(&empty);
        // Implement as empty string or all-zeros hash — be consistent
        assert!(root.is_empty() || root.chars().all(|c| c == '0'));
    }

    #[test]
    #[ignore = "implement merkle_root"]
    fn test_merkle_root_single_item() {
        let items = vec!["tx1"];
        let root = merkle_root(&items);
        // Root of a single item is the hash of that item
        assert_eq!(root, sha256_hex(b"tx1"));
    }

    #[test]
    #[ignore = "implement merkle_root"]
    fn test_merkle_root_two_items() {
        let items = vec!["tx1", "tx2"];
        let root = merkle_root(&items);
        let h1 = sha256_hex(b"tx1");
        let h2 = sha256_hex(b"tx2");
        assert_eq!(root, combine_hashes(&h1, &h2));
    }

    #[test]
    #[ignore = "implement merkle_root"]
    fn test_merkle_root_odd_count_duplicates_last() {
        let items = vec!["tx1", "tx2", "tx3"];
        let root = merkle_root(&items);
        // Should not panic; order must be deterministic
        let root2 = merkle_root(&items);
        assert_eq!(root, root2);
    }

    #[test]
    #[ignore = "implement merkle_root"]
    fn test_merkle_root_order_matters() {
        let a = merkle_root(&["tx1", "tx2"]);
        let b = merkle_root(&["tx2", "tx1"]);
        assert_ne!(a, b);
    }

    // ── combine_hashes ────────────────────────────────────────

    #[test]
    #[ignore = "implement combine_hashes"]
    fn test_combine_hashes_is_deterministic() {
        let c1 = combine_hashes("aaa", "bbb");
        let c2 = combine_hashes("aaa", "bbb");
        assert_eq!(c1, c2);
    }

    #[test]
    #[ignore = "implement combine_hashes"]
    fn test_combine_hashes_not_commutative() {
        // Order matters: combine(a,b) != combine(b,a)
        assert_ne!(combine_hashes("aaa", "bbb"), combine_hashes("bbb", "aaa"));
    }
}
