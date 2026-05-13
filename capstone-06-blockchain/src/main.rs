// capstone-06 — Simple Blockchain
//
// ── HOW IT ALL CONNECTS ───────────────────────────────────────
//
//   merkle::sha256_hex(data)           SHA-256 hash as a hex string
//   merkle::merkle_root(tx_ids)        hash all transaction IDs into one root
//       ↓  used inside Block when it is created
//
//   Transaction::new(sender, receiver, amount)
//   Transaction::coinbase(miner, reward)   ← added automatically when mining
//       ↓
//   Wallet::new()                      generate keypair (private_key + address)
//   wallet.sign_transaction(&mut tx)   attach a hash-based signature
//       ↓
//   Block::new(index, transactions, previous_hash)
//       ↓  sets merkle_root from transactions, nonce = 0, hash = ""
//   mining::mine_block(&mut block, difficulty)
//       ↓  loop: block.nonce++, block.hash = compute_hash()
//       ↓  until block.hash starts with `difficulty` zeros
//
//   Blockchain::new(difficulty)        creates genesis block
//   bc.add_transaction(tx)             add to pending pool
//   bc.mine_pending(miner_address)     bundle pending txs, mine a block, append
//       ↓  repeating this is how the chain grows
//   bc.balance_of(address)             scan all blocks, tally credits/debits
//   bc.validate()                      check hashes + merkle roots link correctly
//   bc.history_of(address)             all txs involving this address
//
// ── IMPLEMENTATION ORDER ──────────────────────────────────────
//
//   1. merkle.rs      — sha256_hex(data), combine_hashes(left, right), merkle_root(items)
//                       Pure hashing. No dependencies. Start here.
//                       Test: sha256_hex(b"") == known constant (64 hex chars).
//
//   2. error.rs       — ChainError variants: InvalidBlock, InvalidHash,
//                       InvalidMerkleRoot, InvalidTransaction, etc.
//
//   3. transaction.rs — Transaction::new, coinbase, id, to_bytes, sign, is_coinbase
//                       id() = sha256(sender + receiver + amount.to_string())
//
//   4. block.rs       — Block::genesis, Block::new, compute_hash, is_valid_proof,
//                       has_valid_hash, has_valid_merkle_root
//                       compute_hash: sha256(index + timestamp + previous_hash
//                                           + merkle_root + nonce)
//
//   5. mining.rs      — mine_block: loop { nonce++; hash = compute_hash(); check }
//                       mine_block_parallel (bonus): rayon splits nonce space
//
//   6. wallet.rs      — Wallet::new (random key), from_private_key, sign, sign_transaction
//                       address = sha256(private_key) — replace with Ed25519 later
//
//   7. chain.rs       — Blockchain::new, add_transaction, mine_pending,
//                       balance_of, validate, history_of, height
//                       mine_pending prepends a coinbase tx before mining.
//
// ── ONCE COMPLETE ─────────────────────────────────────────────

use blockchain::{Blockchain, Transaction, Wallet};
use blockchain::mining::DEFAULT_DIFFICULTY;

fn main() {
    println!("Forge Blockchain — Capstone 6");
    println!();
    println!("Run `cargo test -p capstone-06-blockchain` to track progress.");
    println!("Difficulty target: {} leading zero(s)", DEFAULT_DIFFICULTY);

    // Uncomment once implemented:
    //
    // // Create wallets
    // let alice = Wallet::new();
    // let bob   = Wallet::new();
    // println!("\nAlice address: {}", alice.address());
    // println!("Bob   address: {}", bob.address());
    //
    // // Build the chain
    // let mut bc = Blockchain::new(DEFAULT_DIFFICULTY);
    // println!("Genesis block mined. Height: {}", bc.height()); // 1
    //
    // // Alice mines a block to earn the reward, then sends some to Bob
    // bc.mine_pending(alice.address()).unwrap();
    // println!("Alice balance after mining: {}", bc.balance_of(alice.address())); // 25
    //
    // // Create and sign a transaction
    // let mut tx = Transaction::new(alice.address(), bob.address(), 10);
    // alice.sign_transaction(&mut tx);
    // bc.add_transaction(tx).unwrap();
    //
    // // Mine the pending transaction into a new block
    // bc.mine_pending(alice.address()).unwrap();
    // println!("\nFinal chain height: {}", bc.height()); // 3
    // println!("Alice balance: {}", bc.balance_of(alice.address())); // 25 + 25 - 10 = 40
    // println!("Bob   balance: {}", bc.balance_of(bob.address()));   // 10
    //
    // // Validate the whole chain
    // match bc.validate() {
    //     Ok(())  => println!("\nChain is VALID"),
    //     Err(e)  => println!("\nChain INVALID: {}", e),
    // }
    //
    // // Show Alice's transaction history
    // println!("\nAlice history ({} txs):", bc.history_of(alice.address()).len());
    // for tx in bc.history_of(alice.address()) {
    //     println!("  {} → {} : {} coins", tx.sender, tx.receiver, tx.amount);
    // }
}
