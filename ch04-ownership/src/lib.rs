// ============================================================
// Chapter 4: Ownership — Library Root
// ============================================================
//
// THE THREE OWNERSHIP RULES:
//   1. Each value has exactly one owner.
//   2. There can only be one owner at a time.
//   3. When the owner goes out of scope the value is dropped.

pub mod moves;      // Move semantics
pub mod copies;     // Copy types vs Clone
pub mod heap;       // Box<T>, Rc<T>, Arc<T>
pub mod display;    // Output helpers

pub use moves::demo_moves;
pub use copies::demo_copies;
pub use heap::demo_heap;
