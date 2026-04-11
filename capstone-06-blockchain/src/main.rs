use blockchain::{Blockchain, Transaction, Wallet};
use blockchain::mining::DEFAULT_DIFFICULTY;

fn main() {
    println!("Forge Blockchain — Capstone 6");
    println!();
    println!("Implement the modules in order:");
    println!("  1. src/merkle.rs      — sha256_hex, merkle_root");
    println!("  2. src/transaction.rs — Transaction struct");
    println!("  3. src/block.rs       — Block + compute_hash");
    println!("  4. src/mining.rs      — Proof-of-work loop");
    println!("  5. src/wallet.rs      — Keypair + signing");
    println!("  6. src/chain.rs       — Full blockchain + validation");
    println!();
    println!("Run `cargo test -p capstone-06-blockchain` to see which tests pass.");
    println!("Difficulty target: {} leading zero(s)", DEFAULT_DIFFICULTY);

    // Uncomment once implemented:
    //
    // let alice = Wallet::new();
    // let bob   = Wallet::new();
    // println!("\nAlice: {}", alice.address());
    // println!("Bob:   {}", bob.address());
    //
    // let mut bc = Blockchain::new(DEFAULT_DIFFICULTY);
    // bc.add_transaction(Transaction::new(alice.address(), bob.address(), 50)).unwrap();
    // bc.mine_pending(alice.address()).unwrap();
    //
    // println!("\nChain height: {}", bc.height());
    // println!("Alice balance: {}", bc.balance_of(alice.address()));
    // println!("Bob balance:   {}", bc.balance_of(bob.address()));
    //
    // match bc.validate() {
    //     Ok(())  => println!("\nChain is VALID"),
    //     Err(e)  => println!("\nChain INVALID: {}", e),
    // }
}
