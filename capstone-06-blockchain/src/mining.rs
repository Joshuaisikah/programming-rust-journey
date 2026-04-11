// mining.rs — Proof-of-Work mining
//
// Mining = finding a nonce such that sha256(block_header + nonce)
// starts with `difficulty` zero characters.
//
// Single-threaded: increment nonce until hash satisfies difficulty.
// Multi-threaded (bonus): split nonce space across rayon threads.

use crate::block::Block;
use crate::error::ChainError;

pub const DEFAULT_DIFFICULTY: usize = 3; // leading zeros required
pub const MINING_REWARD: u64 = 25;

/// Mine a block: find a nonce that satisfies the difficulty target.
/// Updates block.nonce and block.hash in place.
pub fn mine_block(block: &mut Block, difficulty: usize) -> Result<(), ChainError> {
    todo!("loop: block.nonce++, block.hash = block.compute_hash(), break if is_valid_proof")
}

/// Parallel mining: try ranges of nonces across multiple threads.
/// Uses rayon to search in parallel; first valid nonce wins.
pub fn mine_block_parallel(block: &mut Block, difficulty: usize) -> Result<(), ChainError> {
    todo!("split nonce space, use rayon::find_any to search concurrently")
}

/// Return the number of hashes per second the current machine can compute (benchmark).
pub fn benchmark_hashrate(duration_secs: u64) -> u64 {
    todo!("loop hashing for duration_secs, count iterations, return count/duration_secs")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::Block;

    // Use difficulty=1 in tests so they run fast (one leading zero)
    const TEST_DIFFICULTY: usize = 1;

    #[test]
    #[ignore = "implement mine_block"]
    fn test_mined_block_satisfies_difficulty() {
        let mut b = Block::genesis();
        mine_block(&mut b, TEST_DIFFICULTY).unwrap();
        assert!(b.is_valid_proof(TEST_DIFFICULTY));
    }

    #[test]
    #[ignore = "implement mine_block"]
    fn test_mined_block_has_valid_hash() {
        let mut b = Block::genesis();
        mine_block(&mut b, TEST_DIFFICULTY).unwrap();
        assert!(b.has_valid_hash());
    }

    #[test]
    #[ignore = "implement mine_block"]
    fn test_mined_block_nonce_is_nonzero_for_difficulty_gt_0() {
        let mut b = Block::genesis();
        mine_block(&mut b, TEST_DIFFICULTY).unwrap();
        // It's astronomically unlikely the nonce=0 hash satisfies difficulty>=1
        // This just verifies mining actually searched
        assert!(b.nonce > 0 || b.hash.starts_with('0'));
    }

    #[test]
    #[ignore = "implement mine_block_parallel"]
    fn test_parallel_mining_satisfies_difficulty() {
        let mut b = Block::genesis();
        mine_block_parallel(&mut b, TEST_DIFFICULTY).unwrap();
        assert!(b.is_valid_proof(TEST_DIFFICULTY));
        assert!(b.has_valid_hash());
    }
}
