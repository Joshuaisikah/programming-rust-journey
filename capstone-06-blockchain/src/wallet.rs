// wallet.rs — Wallet (keypair, address, sign, verify)
//
// A Wallet generates a public/private key pair.
// The public key serves as the wallet's "address".
//
// For simplicity we use a hash-based pseudo-signature scheme:
//   sign(message)   = sha256(private_key + message)   (NOT production-safe)
//   verify(address, message, sig) = sha256(???) — without the private key,
//     real verification requires asymmetric crypto (e.g., Ed25519).
//
// When you implement: replace with a real crypto library such as
// `ed25519-dalek` or `ring` for production-quality signatures.

use sha2::{Digest, Sha256};
use crate::transaction::Transaction;

#[derive(Debug, Clone)]
pub struct Wallet {
    /// The private key (kept secret; never transmitted).
    private_key: String,
    /// The public address (shared; used as sender/receiver identity).
    pub address: String,
}

impl Wallet {
    /// Generate a new wallet with a random private key.
    pub fn new() -> Self {
        todo!("generate random 32-byte private_key, derive address = sha256(private_key)")
    }

    /// Create a wallet from a known private key (for deterministic tests).
    pub fn from_private_key(private_key: impl Into<String>) -> Self {
        todo!("derive address = sha256(private_key.as_bytes())")
    }

    /// Return this wallet's public address.
    pub fn address(&self) -> &str {
        todo!("&self.address")
    }

    /// Sign a byte slice. Returns a hex-encoded signature string.
    pub fn sign(&self, data: &[u8]) -> String {
        todo!("sha256(private_key bytes + data), return as hex — replace with real asymmetric crypto later")
    }

    /// Sign a transaction (signs its canonical byte representation).
    pub fn sign_transaction(&self, tx: &mut Transaction) {
        todo!("tx.sign(self.sign(&tx.to_bytes()))")
    }

    /// Verify a signature given the signer's address (public key hash).
    /// With real asymmetric crypto this is possible; here we just
    /// check that the signature is a non-empty hex string of the right length.
    pub fn verify_signature(address: &str, data: &[u8], signature: &str) -> bool {
        todo!("for real Ed25519: verify(public_key, data, sig); for now: signature.len() == 64")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "implement Wallet::from_private_key"]
    fn test_wallet_address_from_known_key() {
        let w = Wallet::from_private_key("secret");
        // Same key always produces same address
        let w2 = Wallet::from_private_key("secret");
        assert_eq!(w.address(), w2.address());
    }

    #[test]
    #[ignore = "implement Wallet::from_private_key"]
    fn test_different_keys_give_different_addresses() {
        let w1 = Wallet::from_private_key("key1");
        let w2 = Wallet::from_private_key("key2");
        assert_ne!(w1.address(), w2.address());
    }

    #[test]
    #[ignore = "implement Wallet::sign"]
    fn test_sign_returns_hex_string() {
        let w = Wallet::from_private_key("secret");
        let sig = w.sign(b"hello");
        assert_eq!(sig.len(), 64); // SHA-256 hex = 64 chars
        assert!(sig.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    #[ignore = "implement Wallet::sign"]
    fn test_sign_is_deterministic() {
        let w = Wallet::from_private_key("secret");
        assert_eq!(w.sign(b"msg"), w.sign(b"msg"));
    }

    #[test]
    #[ignore = "implement Wallet::sign_transaction"]
    fn test_sign_transaction_attaches_signature() {
        let w = Wallet::from_private_key("alice");
        let mut tx = Transaction::new(w.address(), "Bob", 10);
        w.sign_transaction(&mut tx);
        assert!(tx.signature.is_some());
    }

    #[test]
    #[ignore = "implement Wallet::new"]
    fn test_new_wallet_addresses_differ() {
        let w1 = Wallet::new();
        let w2 = Wallet::new();
        assert_ne!(w1.address(), w2.address());
    }
}
