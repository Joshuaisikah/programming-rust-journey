// chain.rs — The Blockchain
//
// A Blockchain is a Vec<Block> where each block references
// the hash of the previous one. Validity requires:
//   1. Each block's stored hash matches its computed hash
//   2. Each block's previous_hash matches the prior block's hash
//   3. Each block's merkle_root matches its transactions
//   4. Each block's hash satisfies the difficulty (PoW)

use crate::block::Block;
use crate::error::ChainError;
use crate::mining::{mine_block, DEFAULT_DIFFICULTY, MINING_REWARD};
use crate::transaction::Transaction;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
    /// Transactions waiting to be included in the next block.
    pub pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    /// Create a new chain starting with the genesis block.
    pub fn new(difficulty: usize) -> Self {
        todo!("create genesis block, store in blocks: vec![genesis], pending: vec![]")
    }

    /// Return the last block in the chain.
    pub fn last_block(&self) -> &Block {
        todo!("self.blocks.last().unwrap()")
    }

    /// Add a transaction to the pending pool.
    pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), ChainError> {
        todo!("basic validation (non-zero amount), push to pending_transactions")
    }

    /// Mine a new block with all pending transactions.
    /// Adds a coinbase reward transaction to `miner_address`.
    /// Clears pending_transactions after mining.
    pub fn mine_pending(&mut self, miner_address: &str) -> Result<(), ChainError> {
        todo!("prepend coinbase tx, build new block, mine it, push to chain, clear pending")
    }

    /// Return the confirmed balance of `address` by scanning all blocks.
    pub fn balance_of(&self, address: &str) -> u64 {
        todo!("sum all received amounts, subtract all sent amounts across all blocks")
    }

    /// Validate the entire chain. Returns Ok(()) if valid.
    pub fn validate(&self) -> Result<(), ChainError> {
        todo!("check each block: valid hash, valid merkle, correct previous_hash link")
    }

    /// Return all transactions involving `address` (sent or received).
    pub fn history_of(&self, address: &str) -> Vec<&Transaction> {
        todo!("collect txs where sender==address or receiver==address")
    }

    /// Total number of blocks (including genesis).
    pub fn height(&self) -> usize {
        todo!("self.blocks.len()")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chain() -> Blockchain {
        Blockchain::new(1) // difficulty=1 for fast tests
    }

    #[test]
    #[ignore = "implement Blockchain::new"]
    fn test_new_chain_has_genesis_block() {
        let bc = chain();
        assert_eq!(bc.height(), 1);
        assert_eq!(bc.last_block().index, 0);
    }

    #[test]
    #[ignore = "implement Blockchain::add_transaction"]
    fn test_add_transaction_to_pending() {
        let mut bc = chain();
        bc.add_transaction(Transaction::new("Alice", "Bob", 10)).unwrap();
        assert_eq!(bc.pending_transactions.len(), 1);
    }

    #[test]
    #[ignore = "implement Blockchain::add_transaction"]
    fn test_zero_amount_transaction_rejected() {
        let mut bc = chain();
        let result = bc.add_transaction(Transaction::new("Alice", "Bob", 0));
        assert!(result.is_err());
    }

    #[test]
    #[ignore = "implement Blockchain::mine_pending"]
    fn test_mine_adds_block_to_chain() {
        let mut bc = chain();
        bc.add_transaction(Transaction::new("Alice", "Bob", 5)).unwrap();
        bc.mine_pending("Miner").unwrap();
        assert_eq!(bc.height(), 2);
        assert!(bc.pending_transactions.is_empty());
    }

    #[test]
    #[ignore = "implement Blockchain::mine_pending"]
    fn test_mined_block_satisfies_difficulty() {
        let mut bc = chain();
        bc.mine_pending("Miner").unwrap();
        let last = bc.last_block();
        assert!(last.is_valid_proof(bc.difficulty));
    }

    #[test]
    #[ignore = "implement Blockchain::balance_of"]
    fn test_miner_receives_reward() {
        let mut bc = chain();
        bc.mine_pending("Miner").unwrap();
        assert_eq!(bc.balance_of("Miner"), MINING_REWARD);
    }

    #[test]
    #[ignore = "implement Blockchain::balance_of"]
    fn test_balance_reflects_transfers() {
        let mut bc = chain();
        // Give Miner some coins via mining reward first
        bc.mine_pending("Miner").unwrap();
        bc.add_transaction(Transaction::new("Miner", "Alice", 10)).unwrap();
        bc.mine_pending("Miner").unwrap();

        let alice_balance = bc.balance_of("Alice");
        assert_eq!(alice_balance, 10);
    }

    #[test]
    #[ignore = "implement Blockchain::validate"]
    fn test_fresh_chain_is_valid() {
        let bc = chain();
        assert!(bc.validate().is_ok());
    }

    #[test]
    #[ignore = "implement Blockchain::validate"]
    fn test_tampered_block_fails_validation() {
        let mut bc = chain();
        bc.mine_pending("Miner").unwrap();
        // Tamper with a transaction amount
        bc.blocks[1].transactions[0].amount = 999;
        assert!(bc.validate().is_err());
    }

    #[test]
    #[ignore = "implement Blockchain::history_of"]
    fn test_history_includes_sent_and_received() {
        let mut bc = chain();
        bc.add_transaction(Transaction::new("Alice", "Bob", 5)).unwrap();
        bc.mine_pending("Miner").unwrap();

        let alice_history = bc.history_of("Alice");
        assert_eq!(alice_history.len(), 1);
    }
}
