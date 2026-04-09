// ============================================================
// Chapter 5: References & Borrowing — Library Root
// ============================================================
//
// REFERENCE RULES (enforced at compile time):
//   1. At any given time, you can have EITHER:
//        - Any number of shared references (&T), OR
//        - Exactly ONE mutable reference (&mut T)
//   2. References must always be valid (no dangling pointers).
//
// LIFETIME RULES:
//   - Every reference has a lifetime: the scope it is valid in.
//   - The compiler infers lifetimes in most cases (elision rules).
//   - Explicit annotations ('a) are needed when the compiler can't infer.

pub mod borrows;    // Shared &T and mutable &mut T references
pub mod lifetimes;  // Lifetime annotations and elision
pub mod slices;     // String slices (&str) and slice operations
pub mod display;    // Output helpers

pub use borrows::demo_borrows;
pub use lifetimes::demo_lifetimes;
pub use slices::demo_slices;
