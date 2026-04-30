# Capstone 06 — Blockchain

## What You're Building

A minimal but complete blockchain — the same fundamental architecture that
underlies Bitcoin and Ethereum, implemented from scratch in Rust.

```
Block #0 (Genesis)
  transactions: []
  hash: 0000a3f7...
  prev: 0000000000...

Block #1
  transactions: [Joshua → Alice: 50 coins]
  hash: 00001b2c...
  prev: 0000a3f7...
```

Features: SHA-256 proof-of-work, Merkle tree transaction hashing, a transaction
system with sender/receiver/amount, wallets with signing and verification,
chain validation, and multi-threaded mining.

## The Real Point

A blockchain is not magic — it is a linked list plus cryptographic hashing
plus a consensus rule:

- **SHA-256 hashing** — every block contains the hash of the previous block.
  Changing one block invalidates every block after it. Immutability without
  a central authority.

- **Proof-of-work** — mining means finding a hash that starts with enough zeros.
  The only way to find it is brute force: try billions of nonces until one works.
  Rayon makes multiple CPU cores mine in parallel.

- **Merkle trees** — instead of storing every transaction in the block header,
  you hash pairs of transactions recursively until you get one root hash.
  Change any transaction and the root hash changes. You build this tree from scratch.

- **Wallets and signatures** — a wallet is a cryptographic key pair. A transaction
  is signed with the sender's private key and verified with their public key.

## Architecture: How the Files Connect

```
main.rs  (CLI: mine, send, validate, balance)
  │
  └── lib.rs  re-exports: Blockchain, Block, Transaction, Wallet
        │
        ├── chain.rs         ← the Blockchain struct
        │     │                  a Vec<Block> with validation logic
        │     │                  add_block(), is_valid(), get_balance()
        │     │                  the top-level object the CLI talks to
        │     │
        │     ├── block.rs   ← a single Block
        │     │                  index, timestamp, transactions, prev_hash,
        │     │                  nonce, hash
        │     │                  compute_hash() calls sha2 on its own fields
        │     │
        │     └── mining.rs  ← proof-of-work
        │                        mine(block, difficulty) → Block with valid hash
        │                        uses rayon to try nonces in parallel across cores
        │
        ├── transaction.rs   ← Transaction struct
        │                        sender, receiver, amount, signature
        │                        sign(wallet) and verify() live here
        │
        ├── wallet.rs        ← a key pair (private + public key)
        │                        generate(), sign(data), verify(data, sig)
        │                        the only place private keys are handled
        │
        ├── merkle.rs        ← Merkle tree builder
        │                        takes a list of transactions, returns a root hash
        │                        used by block.rs when computing its own hash
        │
        └── error.rs         ← ChainError: InvalidBlock, InvalidSignature,
                                 InsufficientFunds, etc.
                                 used by every module
```

Data flows from bottom to top: `transaction` and `wallet` are base types.
`block` uses both plus `merkle`. `chain` orchestrates `block` and `mining`.
The CLI in `main` only talks to `chain` — it never reaches into lower layers directly.

## Why Each File Exists

**`main.rs`** — The CLI: `mine`, `send`, `validate`, `balance` commands.
Kept thin; all blockchain logic lives in the library so it is fully testable
without running the CLI.

**`block.rs`** — One block's data and its own hash computation. Separated from
`chain.rs` because a block is a self-contained unit — it can hash itself without
knowing about the chain around it. Tests for block hashing run without constructing
an entire chain.

**`chain.rs`** — The blockchain: a `Vec<Block>` plus the rules that govern it.
Separated from `block.rs` because chain-level logic (validation, balance, fork
detection) is a different concern than single-block logic.

**`mining.rs`** — Proof-of-work in isolation. `mine(block, difficulty)` takes an
unmined block and returns a mined one. Separated so the mining algorithm can be
swapped (different difficulty functions, different hash targets) without touching
the block or chain code. Rayon's parallel iterator lives here.

**`transaction.rs`** — Defines what a transaction is and how to sign and verify one.
Separated from `block.rs` because transactions are independent values — a wallet
creates a transaction without knowing what block it will end up in.

**`wallet.rs`** — Key generation, signing, and verification. Separated because
private keys must be handled carefully. Keeping all key material in one file makes
it easy to audit and easy to replace the cryptography library later.

**`merkle.rs`** — The Merkle tree algorithm is a pure function: take transactions,
return a root hash. Separated so it can be tested independently with known inputs
and expected outputs, and so `block.rs` stays focused on block structure, not
hashing algorithms.

**`error.rs`** — All domain errors in one place. Every module returns
`Result<_, ChainError>`. Separated so no module needs to import from another
module just to get an error type.

## Chapters It Combines

Ch09 Structs · Ch11 Traits & Generics · Ch16 Collections ·
Ch18 I/O · Ch22 Unsafe (for low-level crypto operations)

## Mental Model

Bitcoin's source code is ~100,000 lines. Your blockchain is ~1,000 lines.
But the core ideas — linked blocks, proof-of-work, Merkle trees, digital signatures —
are identical. After this capstone you will be able to read Bitcoin's whitepaper
and understand every sentence, because you built what it describes.
