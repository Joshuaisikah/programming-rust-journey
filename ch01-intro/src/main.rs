
fn main() {
    println!("=== RUST MEMORY SAFETY DEMO ===\n");

    demo_stack_allocation();
    demo_heap_allocation();
    demo_ownership_move();
    demo_borrowing();
}
fn demo_stack_allocation() {
    let x: i32 = 42;
    let y: f64 = 3.14;


    println!("Stack Allocation:");
    println!("  x = {} at address: {:p}", x, &x);
    println!("  y = {} at address: {:p}", y, &y);
    println!();
}
fn demo_heap_allocation() {
    let boxed_number = Box::new(100);
    let boxed_string = Box::new(String::from("Hello, Heap!"));

    // Print the data and where it lives
    println!("Heap Allocation:");
    println!("  boxed_number = {}", boxed_number);
    println!("    Stack pointer at: {:p}", &boxed_number);
    println!("    Heap data at: {:p}", &*boxed_string);  // Deref to see heap address
    println!();
}
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