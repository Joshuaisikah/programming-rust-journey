# Chapter 1 Project: Memory Safety Visualizer

Build a program that demonstrates Rust's memory safety guarantees.

---

## 🎯 Learning Goals

- Understand stack vs heap allocation
- See ownership in action
- Demonstrate what Rust prevents at compile time
- Use Box for heap allocation

---

## 📋 Project Structure

```rust
// main.rs structure:

fn main() {
    println!("=== RUST MEMORY SAFETY DEMO ===\n");

    demo_stack_allocation();
    demo_heap_allocation();
    demo_ownership_move();
    demo_borrowing();
}

// Implement these 4 functions below main()
```

---

## 🔨 Implementation Guide

### Function 1: `demo_stack_allocation()`

**Purpose:** Show where local variables live

**Convention:** Use `snake_case` for function names

```rust
fn demo_stack_allocation() {
    // Create local variables (stack allocated)
    let x: i32 = 42;
    let y: f64 = 3.14;

    // Print values AND their memory addresses
    // Use {:p} formatter for pointer addresses
    println!("Stack Allocation:");
    println!("  x = {} at address: {:p}", x, &x);
    println!("  y = {} at address: {:p}", y, &y);
    println!();
}
```

**Key Points:**
- Local variables go on the stack
- Use `&` to get a reference (for printing address)
- Stack is FAST but limited in size

---

### Function 2: `demo_heap_allocation()`

**Purpose:** Show Box for heap allocation

**When to use Box:**
- Data size unknown at compile time
- Large data you want to avoid copying
- Transfer ownership of heap data

```rust
fn demo_heap_allocation() {
    // Box puts data on the heap
    let boxed_number = Box::new(100);
    let boxed_string = Box::new(String::from("Hello, Heap!"));

    // Print the data and where it lives
    println!("Heap Allocation:");
    println!("  boxed_number = {}", boxed_number);
    println!("    Stack pointer at: {:p}", &boxed_number);
    println!("    Heap data at: {:p}", &*boxed_number);  // Deref to see heap address
    println!();
}
```

**Key Points:**
- `Box::new()` allocates on heap
- Variable itself (pointer) is on stack
- Actual data is on heap
- Use `&*` to get reference to heap data

---

### Function 3: `demo_ownership_move()`

**Purpose:** Demonstrate ownership transfer (THE most important concept!)

**Convention:** After a move, original variable is INVALID

```rust
fn demo_ownership_move() {
    println!("Ownership Move:");

    // Create a String (heap allocated)
    let s1 = String::from("Hello, Rust!");
    println!("  Created: s1 = '{}'", s1);

    // Move ownership to s2
    let s2 = s1;  // s1 is now INVALID!
    println!("  Moved to: s2 = '{}'", s2);

    // This would NOT compile (uncomment to see error):
    // println!("  s1 = {}", s1);  // ERROR: value moved!

    println!("  ✓ Rust prevents use-after-move bugs!\n");
}
```

**Key Points:**
- String owns heap data
- Assignment MOVES ownership (not copy)
- Original variable becomes invalid
- This prevents double-free bugs!

**Try this:** Uncomment the error line and run `cargo build` to see the error

---

### Function 4: `demo_borrowing()`

**Purpose:** Show borrowing (using without taking ownership)

**Convention:** Use `&` for immutable borrow, `&mut` for mutable

```rust
fn demo_borrowing() {
    println!("Borrowing:");

    let s = String::from("Hello");
    println!("  Created: s = '{}'", s);

    // Borrow s (don't take ownership)
    let length = calculate_length(&s);

    println!("  Length: {} (borrowed with &s)", length);
    println!("  s is still valid: '{}'", s);
    println!("  ✓ Borrowing lets you use data without taking ownership!\n");
}

// Helper function - takes a reference (borrows)
fn calculate_length(s: &String) -> usize {
    s.len()  // Can use s, but don't own it
}
```

**Key Points:**
- `&String` means "borrow a String"
- Function can read but not take ownership
- Caller still owns the data after function returns
- This is the solution to ownership restrictions!

---

## 🎨 Code Style Conventions

### Naming
- **Functions:** `snake_case` → `demo_stack_allocation()`
- **Variables:** `snake_case` → `boxed_number`
- **Constants:** `SCREAMING_SNAKE_CASE` → `MAX_SIZE`
- **Types/Structs:** `PascalCase` → `MyStruct`

### Formatting
- Use 4 spaces for indentation (not tabs)
- Leave blank line between functions
- Add comments explaining WHY, not WHAT

### Function Order
```rust
fn main() {
    // Main function first
}

// Then helper functions in order they're called
fn demo_stack_allocation() { }
fn demo_heap_allocation() { }
fn demo_ownership_move() { }
fn demo_borrowing() { }
fn calculate_length(s: &String) -> usize { }
```

---

## ✅ Completion Checklist

- [ ] All 4 demo functions implemented
- [ ] Prints memory addresses using `{:p}`
- [ ] Shows ownership move
- [ ] Demonstrates borrowing
- [ ] Code compiles without warnings: `cargo build`
- [ ] Runs successfully: `cargo run`
- [ ] Code follows naming conventions

---

## 🚀 Expected Output

```
=== RUST MEMORY SAFETY DEMO ===

Stack Allocation:
  x = 42 at address: 0x7ffd5c8b9a0c
  y = 3.14 at address: 0x7ffd5c8b9a10

Heap Allocation:
  boxed_number = 100
    Stack pointer at: 0x7ffd5c8b9a18
    Heap data at: 0x55f8c4e6f2a0

Ownership Move:
  Created: s1 = 'Hello, Rust!'
  Moved to: s2 = 'Hello, Rust!'
  ✓ Rust prevents use-after-move bugs!

Borrowing:
  Created: s = 'Hello'
  Length: 5 (borrowed with &s)
  s is still valid: 'Hello'
  ✓ Borrowing lets you use data without taking ownership!
```

---

## 💡 Extension Ideas

Once you finish the basic version:

1. **Add Copy types demo** - Show how integers copy instead of move
2. **Multiple borrows** - Show multiple `&` references are OK
3. **Mutable borrow** - Demonstrate `&mut` with modification
4. **Drop demo** - Show when heap memory is freed

---

## 📚 Key Takeaways

After this project, you should understand:

✅ **Stack** - Fast, automatic, limited size
✅ **Heap** - Slower, manual (but Rust automates!), unlimited size
✅ **Ownership** - Each value has one owner
✅ **Move** - Transfer ownership, old variable invalid
✅ **Borrowing** - Use without owning via `&`
✅ **Safety** - Rust prevents entire classes of bugs at compile time!

---

**Start coding!** Follow the structure above and build it step by step. 🦀
