# Rust for C# Developers - Quick Reference Guide

This guide helps C# developers understand Rust concepts by comparing them to familiar C# patterns.

---

## 🎯 Core Concept Mappings

### Classes vs Structs vs Enums

| C# Concept | Rust Equivalent | Notes |
|------------|-----------------|-------|
| `class Person { }` | `struct Person { }` | Rust structs have no inheritance |
| `public string Name { get; set; }` | `pub name: String` | Fields, not properties |
| `new Person()` | `Person { ... }` or `Person::new()` | Struct literal or constructor pattern |
| `enum Status { OK, Error }` | `enum Status { OK, Error }` | Similar, but Rust enums are much more powerful |
| `interface IRepository` | `trait Repository` | Traits are like interfaces on steroids |
| `abstract class` | ❌ Not in Rust | Use traits instead |
| `inheritance` | ❌ Not in Rust | Use composition and traits |

### Methods and Functions

```csharp
// C#: Instance method
public class Person {
    public void SayHello() {
        Console.WriteLine($"Hello, {this.Name}");
    }
}

// C#: Static method
public static void UtilityMethod() { }

// C#: Extension method
public static string Truncate(this string str, int length) { }
```

```rust
// Rust: Instance method (in impl block)
impl Person {
    pub fn say_hello(&self) {  // &self is like "this" in C#
        println!("Hello, {}", self.name);
    }
}

// Rust: Associated function (like static method)
impl Person {
    pub fn new(name: String) -> Person {  // No &self
        Person { name }
    }
}

// Rust: Trait implementation (like extension but official)
// Methods go in impl block, not inside struct
```

---

## 🔄 Error Handling: Exceptions vs Result

### C# - Try/Catch/Throw

```csharp
public class FileService {
    public string ReadFile(string path) {
        try {
            return File.ReadAllText(path);
        }
        catch (IOException ex) {
            Console.Error.WriteLine($"Error: {ex.Message}");
            throw;
        }
    }
}

// Using it
try {
    var content = ReadFile("file.txt");
    Process(content);
}
catch (Exception ex) {
    HandleError(ex);
}
```

### Rust - Result<T, E>

```rust
pub fn read_file(path: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(path)?;  // ? = propagate error
    Ok(content)
}

// Using it
match read_file("file.txt") {
    Ok(content) => process(content),
    Err(e) => handle_error(e),
}

// Or use ? operator to propagate
let content = read_file("file.txt")?;  // Returns early if error
process(content);
```

### Key Differences

| C# | Rust |
|----|------|
| `throw new Exception("error")` | `return Err("error")` |
| `try { ... } catch (Ex e) { }` | `match result { Ok(v) => ..., Err(e) => ... }` |
| `catch { }` propagates | `?` operator propagates |
| Uncaught exceptions crash | Must handle Result (compiler enforced) |

---

## 📦 String Types: String vs &str

C# has one string type. Rust has two main ones:

| Rust Type | C# Equivalent | Description |
|-----------|---------------|-------------|
| `String` | `string` | Owned, heap-allocated, growable |
| `&str` | `ReadOnlySpan<char>` or `string` reference | Borrowed string slice, immutable view |

```csharp
// C#: Everything is just "string"
string owned = "Hello";
string borrowed = owned;  // Reference copy
```

```rust
// Rust: Two different types
let owned: String = "Hello".to_string();  // Owned
let borrowed: &str = &owned;               // Borrowed reference
let literal: &str = "Hello";               // String literal

// Function signatures
fn takes_ownership(s: String) { }   // Consumes the string
fn borrows(s: &str) { }             // Just looks at it
```

**Rule of thumb**: Use `&str` for parameters (like `in` parameters in C#), `String` for owned data.

---

## 🧩 LINQ vs Iterators

Rust iterators are similar to C# LINQ, but they're zero-cost abstractions!

```csharp
// C# LINQ
var results = items
    .Where(x => x.IsActive)
    .Select(x => x.Name)
    .OrderBy(name => name)
    .ToList();
```

```rust
// Rust iterators
let results: Vec<String> = items
    .iter()                              // Start iteration
    .filter(|x| x.is_active)            // Where
    .map(|x| x.name.clone())            // Select
    .collect();                          // ToList

// Note: .sort() is done on Vec, not in chain
```

### Method Mappings

| C# LINQ | Rust Iterator | Notes |
|---------|---------------|-------|
| `.Where(x => ...)` | `.filter(\|x\| ...)` | Filter items |
| `.Select(x => ...)` | `.map(\|x\| ...)` | Transform items |
| `.SelectMany()` | `.flat_map()` | Flatten nested collections |
| `.FirstOrDefault()` | `.find()` | Returns `Option<T>` |
| `.Any(x => ...)` | `.any(\|x\| ...)` | Check if any match |
| `.All(x => ...)` | `.all(\|x\| ...)` | Check if all match |
| `.Count()` | `.count()` | Count items |
| `.ToList()` | `.collect()` | Collect into collection |
| `.Take(n)` | `.take(n)` | Take first n items |
| `.Skip(n)` | `.skip(n)` | Skip first n items |

**Key difference**: Rust iterators are lazy and zero-cost (compile to same code as loops)!

---

## 🎭 Pattern Matching: switch vs match

```csharp
// C# 8+ switch expression
var result = status switch {
    Status.OK => "Success",
    Status.Error => "Failed",
    _ => "Unknown"
};

// C# if with type check
if (obj is Error error) {
    Console.WriteLine(error.Message);
}
```

```rust
// Rust match (compiler ensures exhaustiveness!)
let result = match status {
    Status::OK => "Success",
    Status::Error => "Failed",
    // Compiler error if you forget a case!
};

// Rust if let (like C# "if is" pattern)
if let Err(e) = result {
    println!("{}", e);
}
```

---

## 🔒 Ownership & Borrowing (NEW CONCEPT!)

This is the BIG difference between C# and Rust. C# has garbage collection; Rust has ownership.

### The Rules

1. **Each value has one owner**
2. **When the owner goes out of scope, value is dropped**
3. **You can borrow references (&T) or mutable references (&mut T)**

```rust
// Ownership
let s1 = String::from("hello");
let s2 = s1;  // s1 is MOVED to s2, s1 is now invalid!
// println!("{}", s1);  // ERROR! Can't use s1 anymore

// Borrowing (like passing by reference in C#)
let s1 = String::from("hello");
let len = calculate_length(&s1);  // Borrow s1
println!("{}", s1);  // Still valid! We only borrowed it

fn calculate_length(s: &String) -> usize {  // &String = borrow
    s.len()
}  // s goes out of scope, but doesn't drop because it's a borrow

// Mutable borrowing (like ref parameters in C#)
let mut s = String::from("hello");
change(&mut s);

fn change(s: &mut String) {
    s.push_str(", world");
}
```

**C# analogy**:
- Ownership transfer = moving object to new scope (caller can't use it anymore)
- Immutable borrow (&T) = passing `in` parameter (read-only)
- Mutable borrow (&mut T) = passing `ref` parameter (can modify)

---

## 🏗️ Traits vs Interfaces

Traits are like C# interfaces, but more powerful!

```csharp
// C# Interface
public interface IDrawable {
    void Draw();
}

public class Circle : IDrawable {
    public void Draw() {
        Console.WriteLine("Drawing circle");
    }
}
```

```rust
// Rust Trait
pub trait Drawable {
    fn draw(&self);
}

pub struct Circle;

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle");
    }
}
```

### Key Differences

| C# Interface | Rust Trait |
|--------------|------------|
| Classes implement interfaces | Types implement traits |
| `class X : IFoo` | `impl Foo for X` |
| Can't add interface to existing type | Can add trait to any type (even from other libs!) |
| Can have default implementations (C# 8+) | Can have default implementations |

---

## 🧪 Testing

```csharp
// C# with xUnit/NUnit
[Test]
public void TestSearch() {
    var result = Search("pattern", "content");
    Assert.Equal(2, result.Count);
}
```

```rust
// Rust built-in testing
#[test]
fn test_search() {
    let result = search("pattern", "content");
    assert_eq!(2, result.len());
}

// Run with: cargo test
```

---

## 📝 Common Patterns Translation

### Null Checks

```csharp
// C#
if (value != null) {
    Use(value);
}

var result = value ?? "default";
```

```rust
// Rust (no null! Use Option<T>)
if let Some(v) = value {
    use_value(v);
}

let result = value.unwrap_or("default");
```

### Collections

```csharp
// C#
var list = new List<int> { 1, 2, 3 };
var dict = new Dictionary<string, int>();
```

```rust
// Rust
let vec = vec![1, 2, 3];  // Like List<T>
use std::collections::HashMap;
let mut map = HashMap::new();  // Like Dictionary<K,V>
```

### String Formatting

```csharp
// C#
Console.WriteLine($"Hello, {name}!");
var s = $"Value: {value}";
```

```rust
// Rust
println!("Hello, {}!", name);
let s = format!("Value: {}", value);
```

---

## 🎓 Mental Model Shifts

### From C# to Rust

1. **No null** → Use `Option<T>` (Some/None)
2. **No exceptions** → Use `Result<T, E>` (Ok/Err)
3. **No garbage collector** → Ownership & borrowing
4. **No inheritance** → Composition + traits
5. **No async/await (yet in our chapter)** → But Rust has it! (async/await keywords)
6. **Immutable by default** → Add `mut` keyword for mutability
7. **Structs, not classes** → Data + impl blocks
8. **Modules, not namespaces** → Files are modules

---

## ⚡ Quick Command Reference

| Task | C# | Rust |
|------|----|----|
| Create project | `dotnet new console` | `cargo new project_name` |
| Build | `dotnet build` | `cargo build` |
| Run | `dotnet run` | `cargo run` |
| Test | `dotnet test` | `cargo test` |
| Add package | `dotnet add package` | `cargo add` or edit Cargo.toml |
| Format code | IDE | `cargo fmt` |
| Lint | Roslyn analyzers | `cargo clippy` |

---

## 🎯 Key Takeaways

1. **Rust is explicit** - What's implicit in C# (GC, null, exceptions) is explicit in Rust
2. **Compiler is your friend** - It catches bugs C# compilers miss (null refs, race conditions)
3. **No runtime overhead** - Zero-cost abstractions (LINQ-like perf without foreach loops!)
4. **Different but familiar** - Many concepts map 1:1, just with different syntax

---

## 📚 Next Steps

As you implement the mini grep project:
1. Start with `error.rs` - understand Result vs exceptions
2. Move to `config.rs` - see structs vs classes
3. Try `search.rs` - practice with lifetimes and iterators
4. Wire up `lib.rs` - understand modules
5. Finish with `main.rs` - see it all come together

**Remember**: The code comments in each file have detailed C# comparisons!

Happy coding! 🦀
