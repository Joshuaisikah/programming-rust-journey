# Chapter 3 Project: Programmer's Calculator

Build a calculator that handles multiple number bases, bitwise operations, and conversions.

---

## 🎯 Learning Goals

- Master integer types (i8, u8, i32, u32, i64, u64, usize)
- Understand float types (f32, f64) and precision
- Parse different number bases (binary, hex, octal)
- Perform bitwise operations
- Handle type conversions safely
- Work with char and string types

---

## 📋 Project Structure

```rust
// Recommended organization:

use std::env;

fn main() {
    // Parse command-line args
    // Match command type
    // Call appropriate handler
}

// Number parsing
fn parse_number(s: &str) -> i64 { }

// Operations
fn calculate(a: i64, op: &str, b: i64) { }
fn convert_base(num: i64, to_base: &str) { }
fn convert_temperature(value: f64, from: &str, to: &str) { }

// Display
fn display_in_all_bases(num: i64) { }
fn print_usage() { }

// Tests
#[cfg(test)]
mod tests { }
```

---

## 🔨 Implementation Guide

### Part 1: Parse Different Number Bases

**Why this matters:** Programmers work with hex (0xFF), binary (0b1010), etc.

**Convention:** Use prefixes to identify base
- `0b` or `0B` → Binary
- `0x` or `0X` → Hexadecimal
- `0o` or `0O` → Octal
- No prefix → Decimal

```rust
fn parse_number(s: &str) -> i64 {
    if s.starts_with("0b") || s.starts_with("0B") {
        // Binary: 0b1010 → 10
        // Use i64::from_str_radix to parse
        // radix 2 = binary
        i64::from_str_radix(&s[2..], 2)
            .expect("Invalid binary number")
    } else if s.starts_with("0x") || s.starts_with("0X") {
        // Hexadecimal: 0xFF → 255
        // radix 16 = hexadecimal
        i64::from_str_radix(&s[2..], 16)
            .expect("Invalid hexadecimal number")
    } else if s.starts_with("0o") || s.starts_with("0O") {
        // Octal: 0o77 → 63
        // radix 8 = octal
        i64::from_str_radix(&s[2..], 8)
            .expect("Invalid octal number")
    } else {
        // Decimal: 42 → 42
        s.parse::<i64>()
            .expect("Invalid decimal number")
    }
}
```

**Key Points:**
- `&s[2..]` skips first 2 characters (the prefix)
- `from_str_radix(string, radix)` parses with given base
- Use `i64` for large number support (-9 quintillion to +9 quintillion)
- `.expect()` panics on error with message

**Type Choice:**
- Use `i64` for signed integers (can be negative)
- Use `u64` if only positive numbers needed
- Use `usize` for array indices and sizes

---

### Part 2: Arithmetic Operations

**Convention:** Match on operator string

```rust
fn calculate(a: i64, op: &str, b: i64) {
    let result = match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,        // Integer division
        "%" => a % b,        // Modulo (remainder)
        _ => {
            eprintln!("Unknown operator: {}", op);
            return;
        }
    };

    println!("\nResult: {}", result);
    display_in_all_bases(result);
}
```

**Key Points:**
- Match is exhaustive - must handle all cases
- `_` is catch-all pattern
- Return early on error
- Integer division truncates: 7 / 2 = 3

**Overflow handling:**
```rust
// For production code, consider:
let result = a.checked_add(b).expect("Overflow!");
let result = a.saturating_add(b);  // Clamps at max
let result = a.wrapping_add(b);    // Wraps around
```

---

### Part 3: Bitwise Operations

**Why:** Essential for low-level programming, flags, hardware

**Convention:** Use descriptive names or symbols

```rust
fn calculate(a: i64, op: &str, b: i64) {
    let result = match op {
        // Arithmetic (from before)
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        "%" => a % b,

        // Bitwise operations
        "AND" | "&" => a & b,     // Bitwise AND
        "OR"  | "|" => a | b,     // Bitwise OR
        "XOR" | "^" => a ^ b,     // Bitwise XOR
        "<<" => a << b,           // Left shift
        ">>" => a >> b,           // Right shift

        _ => {
            eprintln!("Unknown operator: {}", op);
            return;
        }
    };

    println!("\nResult: {}", result);
    display_in_all_bases(result);
}
```

**Bitwise Operations Explained:**
- `&` (AND) - Both bits must be 1
- `|` (OR) - At least one bit must be 1
- `^` (XOR) - Bits must be different
- `<<` - Shift bits left (multiply by 2)
- `>>` - Shift bits right (divide by 2)

**Example:**
```
  1100  (12)
& 1010  (10)
------
  1000  (8)
```

---

### Part 4: Display in All Bases

**Convention:** Use Rust's built-in formatting

```rust
fn display_in_all_bases(num: i64) {
    println!("  Decimal:  {}", num);
    println!("  Hex:      0x{:X}", num);      // Uppercase hex
    println!("  Binary:   0b{:b}", num);
    println!("  Octal:    0o{:o}", num);
}
```

**Format Specifiers:**
- `{}` - Default (decimal)
- `{:X}` - Uppercase hexadecimal
- `{:x}` - Lowercase hexadecimal
- `{:b}` - Binary
- `{:o}` - Octal
- `{:08b}` - Binary with 8 digits (padded)

---

### Part 5: Base Conversion

**Convention:** Accept human-friendly names

```rust
fn convert_base(num: i64, to_base: &str) {
    match to_base.to_lowercase().as_str() {
        "binary" | "bin" | "b" => println!("0b{:b}", num),
        "hex" | "hexadecimal" | "h" => println!("0x{:X}", num),
        "octal" | "oct" | "o" => println!("0o{:o}", num),
        "decimal" | "dec" | "d" => println!("{}", num),
        _ => eprintln!("Unknown base: {}", to_base),
    }
}
```

**Key Points:**
- `.to_lowercase()` for case-insensitive matching
- Multiple patterns with `|`
- Support abbreviations for convenience

---

### Part 6: Temperature Conversion

**Why:** Practice with float types and precision

**Convention:** Use `f64` for better precision than `f32`

```rust
fn convert_temperature(value: f64, from: &str, to: &str) {
    // Convert everything to Celsius first
    let celsius = match from.to_uppercase().as_str() {
        "C" => value,
        "F" => (value - 32.0) * 5.0 / 9.0,
        "K" => value - 273.15,
        _ => {
            eprintln!("Unknown temperature unit: {}", from);
            return;
        }
    };

    // Then convert from Celsius to target
    let result = match to.to_uppercase().as_str() {
        "C" => celsius,
        "F" => (celsius * 9.0 / 5.0) + 32.0,
        "K" => celsius + 273.15,
        _ => {
            eprintln!("Unknown temperature unit: {}", to);
            return;
        }
    };

    // Print with 2 decimal places
    println!("{:.2}°{} = {:.2}°{}",
        value, from.to_uppercase(),
        result, to.to_uppercase()
    );
}
```

**Key Points:**
- Use `f64` for floating-point (64-bit, more precise than f32)
- Use `.0` suffix for float literals: `32.0`, `9.0`
- `{:.2}` formats with 2 decimal places
- Convert through intermediate unit (Celsius)

**Float Gotchas:**
```rust
// Floats are not always exact!
0.1 + 0.2 == 0.3  // FALSE in most languages!

// Use approximate comparison
(a - b).abs() < 0.00001
```

---

### Part 7: Main Function

**Convention:** Parse args, dispatch to handlers

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "convert" => {
            if args.len() < 4 {
                eprintln!("Usage: cargo run convert <number> <base>");
                return;
            }
            let num = parse_number(&args[2]);
            convert_base(num, &args[3]);
        }
        "temp" => {
            if args.len() < 5 {
                eprintln!("Usage: cargo run temp <value> <from> <to>");
                return;
            }
            let value = args[2].parse::<f64>().expect("Invalid number");
            convert_temperature(value, &args[3], &args[4]);
        }
        _ => {
            // Default: arithmetic operation
            if args.len() < 4 {
                eprintln!("Usage: cargo run <num1> <op> <num2>");
                return;
            }
            let num1 = parse_number(&args[1]);
            let op = &args[2];
            let num2 = parse_number(&args[3]);
            calculate(num1, op, num2);
        }
    }
}
```

---

## 🧪 Testing

**Test each number type and conversion:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_binary() {
        assert_eq!(parse_number("0b1010"), 10);
        assert_eq!(parse_number("0b1111"), 15);
        assert_eq!(parse_number("0b0"), 0);
    }

    #[test]
    fn test_parse_hex() {
        assert_eq!(parse_number("0xFF"), 255);
        assert_eq!(parse_number("0x10"), 16);
        assert_eq!(parse_number("0xABC"), 2748);
    }

    #[test]
    fn test_parse_octal() {
        assert_eq!(parse_number("0o10"), 8);
        assert_eq!(parse_number("0o77"), 63);
        assert_eq!(parse_number("0o100"), 64);
    }

    #[test]
    fn test_parse_decimal() {
        assert_eq!(parse_number("42"), 42);
        assert_eq!(parse_number("0"), 0);
        assert_eq!(parse_number("999"), 999);
    }

    #[test]
    #[should_panic]
    fn test_invalid_binary() {
        parse_number("0b102");  // 2 is not valid in binary!
    }
}
```

---

## 🎨 Code Style Conventions

### Type Selection Guide

| Use Case | Type | Why |
|----------|------|-----|
| Small positive | `u8`, `u16`, `u32` | 0 to max, saves memory |
| Small signed | `i8`, `i16`, `i32` | Can be negative |
| Default integer | `i32` | Rust's default |
| Large numbers | `i64`, `u64` | Wide range |
| Array indices | `usize` | Matches pointer size |
| Floating point | `f64` | More precise than f32 |
| Single character | `char` | Unicode scalar value |

### Number Literals
```rust
let decimal = 1_000_000;     // Underscores for readability
let hex = 0xFF;              // Hexadecimal
let binary = 0b1111_0000;    // Binary with underscores
let float = 3.14_f64;        // Explicit f64
let byte = b'A';             // ASCII byte (u8)
```

### Type Conversions
```rust
let x: i32 = 42;

// Safe conversions
let y = x as i64;            // Widen (always safe)

// Potentially unsafe
let z = x as i8;             // Narrow (can overflow!)

// Safe with checking
let w = i8::try_from(x);     // Returns Result
```

---

## ✅ Completion Checklist

### Basic Features
- [ ] Parse binary numbers (0b)
- [ ] Parse hexadecimal (0x)
- [ ] Parse octal (0o)
- [ ] Parse decimal
- [ ] Arithmetic operations (+, -, *, /, %)
- [ ] Bitwise operations (&, |, ^, <<, >>)

### Conversions
- [ ] Convert to any base
- [ ] Temperature conversions (C, F, K)
- [ ] Display results in all bases

### Quality
- [ ] All tests pass
- [ ] No compiler warnings
- [ ] Proper error messages
- [ ] Usage help

---

## 🚀 Usage Examples

```bash
# Arithmetic with different bases
cargo run 0xFF + 42              # 255 + 42 = 297
cargo run 0b1010 + 0b0101        # 10 + 5 = 15
cargo run 0o77 + 1               # 63 + 1 = 64

# Bitwise operations
cargo run 0b1100 AND 0b1010      # 12 & 10 = 8
cargo run 12 OR 10               # 12 | 10 = 14
cargo run 5 XOR 3                # 5 ^ 3 = 6
cargo run 8 "<<" 2               # 8 << 2 = 32

# Base conversions
cargo run convert 42 binary      # 0b101010
cargo run convert 255 hex        # 0xFF
cargo run convert 0xFF decimal   # 255

# Temperature
cargo run temp 100 C F           # 212°F
cargo run temp 0 C K             # 273.15 K
cargo run temp 98.6 F C          # 37°C

# Run tests
cargo test
```

---

## 💡 Extension Ideas

### Level 1 (Practice More Types)
- [ ] Add `u8`, `u16` range checking
- [ ] Show binary with underscores: `1111_0000`
- [ ] Add char/ASCII conversion

### Level 2 (More Operations)
- [ ] Scientific functions (sqrt, pow)
- [ ] Bit manipulation (count 1s, reverse)
- [ ] More unit conversions (length, weight)

### Level 3 (Advanced)
- [ ] Expression parsing: `(2 + 3) * 4`
- [ ] Variable storage: `let x = 42`
- [ ] Interactive REPL mode

---

## 📚 Key Takeaways

After this project, you understand:

✅ **Integer types** - i8, u8, i32, u32, i64, u64, usize
✅ **Float types** - f32, f64 and precision issues
✅ **Number bases** - Binary, octal, decimal, hexadecimal
✅ **Parsing** - `from_str_radix()`, `.parse()`
✅ **Bitwise ops** - &, |, ^, <<, >>
✅ **Type conversions** - `as`, `from()`, `try_from()`
✅ **Formatting** - `{:X}`, `{:b}`, `{:o}`, `{:.2}`

---

**Start coding!** Build the calculator step by step. Test each part as you go! 🧮
