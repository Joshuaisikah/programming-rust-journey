// block.rs — Block structure
//
// Each Block contains:
//   - index        : position in the chain
//   - timestamp    : unix epoch seconds
//   - transactions : list of transactions committed in this block
//   - previous_hash: hash of the preceding block (links the chain)
//   - merkle_root  : Merkle tree root of transaction IDs
//   - nonce        : proof-of-work value
//   - hash         : SHA-256 of the block header fields
//
// The hash must start with `difficulty` leading zeros (proof-of-work).

use serde::{Deserialize, Serialize};
use crate::transaction::Transaction;
use crate::merkle;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub merkle_root: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    /// Create the genesis block (index 0, no previous hash).
    pub fn genesis() -> Self {
        todo!("Block with index=0, previous_hash=\"0000...0\", compute its hash")
    }

    /// Build a new unmined block (nonce=0, hash not yet valid).
    pub fn new(
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
    ) -> Self {
        todo!("set timestamp to now, compute merkle_root, set nonce=0, hash=\"\"")
    }

    /// Compute the SHA-256 hash of this block's header fields.
    /// Does NOT include self.hash in the input (otherwise circular).
    pub fn compute_hash(&self) -> String {
        todo!("sha256_hex of (index + timestamp + previous_hash + merkle_root + nonce)")
    }

    /// Return true if this block's hash satisfies the given difficulty
    /// (i.e., starts with `difficulty` '0' characters).
    pub fn is_valid_proof(&self, difficulty: usize) -> bool {
        todo!("self.hash.starts_with(&\"0\".repeat(difficulty))")
    }

    /// Return true if the stored hash matches a fresh computation.
    pub fn has_valid_hash(&self) -> bool {
        todo!("self.hash == self.compute_hash()")
    }

    /// Return true if the merkle_root field matches the transactions.
    pub fn has_valid_merkle_root(&self) -> bool {
        todo!("compute merkle_root from transactions, compare with self.merkle_root")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "implement Block::genesis"]
    fn test_genesis_index_is_zero() {
        let b = Block::genesis();
        assert_eq!(b.index, 0);
    }

    #[test]
    #[ignore = "implement Block::genesis"]
    fn test_genesis_previous_hash_is_zeros() {
        let b = Block::genesis();
        assert!(b.previous_hash.chars().all(|c| c == '0'));
    }

    #[test]
    #[ignore = "implement Block::compute_hash"]
    fn test_compute_hash_is_deterministic() {
        let b = Block::genesis();
        assert_eq!(b.compute_hash(), b.compute_hash());
    }

    #[test]
    #[ignore = "implement Block::compute_hash"]
    fn test_compute_hash_length_is_64() {
        let b = Block::genesis();
        assert_eq!(b.compute_hash().len(), 64);
    }

    #[test]
    #[ignore = "implement Block::has_valid_hash"]
    fn test_has_valid_hash_after_genesis() {
        let b = Block::genesis();
        assert!(b.has_valid_hash());
    }

    #[test]
    #[ignore = "implement Block::is_valid_proof"]
    fn test_is_valid_proof_trivial_difficulty() {
        let mut b = Block::genesis();
        // Any hash satisfies difficulty=0
        b.hash = "abcdef".to_string();
        assert!(b.is_valid_proof(0));
    }

    #[test]
    #[ignore = "implement Block::is_valid_proof"]
    fn test_is_valid_proof_requires_leading_zeros() {
        let mut b = Block::genesis();
        b.hash = "00abcdef".to_string();
        assert!(b.is_valid_proof(2));
        assert!(!b.is_valid_proof(3));
    }

    #[test]
    #[ignore = "implement Block::has_valid_merkle_root"]
    fn test_empty_block_has_valid_merkle_root() {
        let b = Block::genesis();
        assert!(b.has_valid_merkle_root());
    }

    #[test]
    #[ignore = "implement Block::new / has_valid_merkle_root"]
    fn test_block_with_transactions_has_valid_merkle_root() {
        let tx = vec![
            Transaction::new("Alice", "Bob", 10),
            Transaction::new("Bob", "Carol", 5),
        ];
        let b = Block::new(1, tx, "0".repeat(64));
        assert!(b.has_valid_merkle_root());
    }
}
