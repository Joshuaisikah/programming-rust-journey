# Chapter 15 Project: Custom Iterator Adapters

Build your own iterator combinators.

```rust
let result = vec![1,2,3]
    .iter()
    .my_filter(|x| x > 1)
    .my_map(|x| x * 2)
    .collect();
```

Master iterators! 🔄
