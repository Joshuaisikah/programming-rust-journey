// Capstone 6 — Simple Blockchain
//
// A minimal but complete blockchain with:
//   - SHA-256 proof-of-work
//   - Merkle tree transaction hashing
//   - Transaction system with sender/receiver/amount
//   - Wallet (keypair, sign, verify)
//   - Chain validation
//   - Multi-threaded mining (rayon)
//   - Simple CLI

pub mod block;
pub mod chain;
pub mod error;
pub mod merkle;
pub mod mining;
pub mod transaction;
pub mod wallet;

pub use block::Block;
pub use chain::Blockchain;
pub use error::ChainError;
pub use transaction::Transaction;
pub use wallet::Wallet;
