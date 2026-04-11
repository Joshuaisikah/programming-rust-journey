// error.rs — Blockchain error types

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChainError {
    #[error("Invalid block hash at index {0}: expected proof-of-work not satisfied")]
    InvalidHash(usize),

    #[error("Broken chain link at index {0}: previous_hash mismatch")]
    BrokenLink(usize),

    #[error("Invalid Merkle root at block {0}")]
    InvalidMerkleRoot(usize),

    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),

    #[error("Insufficient balance: have {have}, need {need}")]
    InsufficientBalance { have: u64, need: u64 },

    #[error("Invalid signature for transaction {0}")]
    InvalidSignature(String),

    #[error("Mining failed: {0}")]
    MiningFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_broken_link_message() {
        let e = ChainError::BrokenLink(3);
        assert!(e.to_string().contains("3"));
    }

    #[test]
    fn test_insufficient_balance_message() {
        let e = ChainError::InsufficientBalance { have: 10, need: 50 };
        let s = e.to_string();
        assert!(s.contains("10"));
        assert!(s.contains("50"));
    }
}
