# Chapter 10 Project: State Machine (Traffic Light / Game)

Build a state machine using enums and pattern matching.

```rust
let mut light = TrafficLight::Red;
light = light.next();  // -> Green

match light {
    TrafficLight::Red => println!("Stop!"),
    TrafficLight::Green => println!("Go!"),
    _ => println!("Caution"),
}
```

## Features

- Enum variants with data
- Pattern matching
- State transitions
- Option/Result mastery

Master enums and patterns! 🎯
