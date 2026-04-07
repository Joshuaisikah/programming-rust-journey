# Chapter 21 Project: Custom Derive Macro

Build your own #[derive(MyTrait)] macro!

```rust
#[derive(Builder)]
struct User {
    name: String,
    age: u32,
}

let user = User::builder()
    .name("Alice")
    .age(25)
    .build();
```

Master macros! 🪄
