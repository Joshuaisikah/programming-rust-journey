// transaction.rs — Transaction model
//
// A Transaction records the transfer of `amount` coins from `sender`
// to `receiver`. In a real chain it would carry a digital signature;
// here we store a simple hash-based ID and an optional signature field.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    /// Sender's public-key string (or "COINBASE" for mining rewards).
    pub sender: String,
    /// Receiver's public-key string.
    pub receiver: String,
    /// Amount transferred.
    pub amount: u64,
    /// Optional base64/hex-encoded signature.
    pub signature: Option<String>,
}

impl Transaction {
    /// Create a new unsigned transaction.
    pub fn new(sender: impl Into<String>, receiver: impl Into<String>, amount: u64) -> Self {
        todo!("Transaction {{ sender: sender.into(), receiver: receiver.into(), amount, signature: None }}")
    }

    /// Create a coinbase transaction (mining reward, no sender).
    pub fn coinbase(receiver: impl Into<String>, reward: u64) -> Self {
        todo!("Transaction::new(\"COINBASE\", receiver, reward)")
    }

    /// Compute a deterministic ID for this transaction (SHA-256 of sender+receiver+amount).
    pub fn id(&self) -> String {
        todo!("sha256(sender + receiver + amount.to_string()), return as hex string")
    }

    /// Serialize this transaction to a canonical byte representation for signing/hashing.
    pub fn to_bytes(&self) -> Vec<u8> {
        todo!("serde_json::to_vec(self).unwrap()")
    }

    /// Attach a signature string.
    pub fn sign(&mut self, signature: String) {
        todo!("self.signature = Some(signature)")
    }

    /// True if this is a coinbase (no-sender) transaction.
    pub fn is_coinbase(&self) -> bool {
        todo!("self.sender == \"COINBASE\"")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "implement Transaction::new"]
    fn test_new_transaction_fields() {
        let tx = Transaction::new("Alice", "Bob", 50);
        assert_eq!(tx.sender, "Alice");
        assert_eq!(tx.receiver, "Bob");
        assert_eq!(tx.amount, 50);
        assert!(tx.signature.is_none());
    }

    #[test]
    #[ignore = "implement Transaction::coinbase"]
    fn test_coinbase_sender() {
        let tx = Transaction::coinbase("Miner", 25);
        assert!(tx.is_coinbase());
        assert_eq!(tx.amount, 25);
    }

    #[test]
    #[ignore = "implement Transaction::id"]
    fn test_id_is_deterministic() {
        let tx = Transaction::new("Alice", "Bob", 100);
        assert_eq!(tx.id(), tx.id()); // same call produces same result
    }

    #[test]
    #[ignore = "implement Transaction::id"]
    fn test_different_transactions_have_different_ids() {
        let t1 = Transaction::new("Alice", "Bob", 10);
        let t2 = Transaction::new("Alice", "Bob", 20);
        assert_ne!(t1.id(), t2.id());
    }

    #[test]
    #[ignore = "implement Transaction::sign"]
    fn test_sign_attaches_signature() {
        let mut tx = Transaction::new("Alice", "Bob", 5);
        tx.sign("fake_sig".into());
        assert_eq!(tx.signature, Some("fake_sig".into()));
    }

    #[test]
    #[ignore = "implement Transaction::is_coinbase"]
    fn test_is_coinbase_false_for_normal() {
        let tx = Transaction::new("Alice", "Bob", 1);
        assert!(!tx.is_coinbase());
    }
}
