# Programming Rust Journey 📘

**A comprehensive chapter-by-chapter journey through "Programming Rust" (2nd Edition)**

> "Programming Rust gives you the big picture. It's your complete guide to the language, from fundamentals to advanced features."

## 📚 About This Repository

This is your **complete learning workspace** for mastering **"Programming Rust"** by Jim Blandy, Jason Orendorff, and Leonora F.S. Tindall (O'Reilly, 2021, ~700 pages).

### 🎯 Structure

**23 chapter projects** - One project for each chapter of the book:

1. **ch01-intro** - Systems programming foundations
2. **ch02-tour** - Tour of Rust basics
3. **ch03-fundamental-types** - Integers, floats, booleans, chars
4. **ch04-ownership** - Understanding ownership
5. **ch05-references** - Borrowing and lifetimes
6. **ch06-expressions** - Expression-oriented programming
7. **ch07-error-handling** - Result, Option, panic
8. **ch08-crates-modules** - Code organization
9. **ch09-structs** - Structured data
10. **ch10-enums-patterns** - Enums and pattern matching
11. **ch11-traits-generics** - Polymorphism in Rust
12. **ch12-operator-overloading** - Custom operators
13. **ch13-utility-traits** - Drop, Copy, Clone, etc.
14. **ch14-closures** - Anonymous functions
15. **ch15-iterators** - Iterator patterns
16. **ch16-collections** - Vec, HashMap, etc.
17. **ch17-strings-text** - String handling
18. **ch18-input-output** - File I/O, networking
19. **ch19-concurrency** - Threads, channels, mutexes
20. **ch20-async** - Async/await programming
21. **ch21-macros** - Declarative and procedural macros
22. **ch22-unsafe** - Unsafe Rust (library project)
23. **ch23-ffi** - Foreign function interface (library project)

---

## 🎓 Learning Approach

### **For Each Chapter:**

```markdown
1. Read the chapter in "Programming Rust"
2. Take notes in [chapter]/NOTES.md
3. Build the exercises in [chapter]/src
4. Run tests: cargo test -p ch[XX]-[name]
5. Document what you learned
```

### **Progressive Mastery:**

- **Ch 1-8:** Foundations (ownership, types, error handling)
- **Ch 9-13:** Intermediate (structs, traits, operators)
- **Ch 14-18:** Advanced patterns (closures, iterators, I/O)
- **Ch 19-23:** Systems programming (concurrency, async, unsafe, FFI)

---

## 🔗 Companion Repository

This repository focuses on **comprehensive foundation** from Programming Rust.

For **advanced production patterns**, see:
**[rustaceans-odyssey](https://github.com/Joshuaisikah/rustaceans-odyssey)** - 5 production-grade projects for "Rust for Rustaceans"

### Two-Book Strategy:

- **Programming Rust** (this repo) → Learn concepts thoroughly
- **Rust for Rustaceans** (other repo) → Master advanced patterns

---

## 🚀 Getting Started

### **Quick Start:**

```bash
cd ~/Rust/programming-rust-journey

# Read START_HERE.md
cat START_HERE.md

# Start with Chapter 1
cd ch01-intro
cat README.md

# Work through exercises
cargo run
cargo test
```

### **Daily Workflow:**

```bash
# 1. Read book chapter
# 2. Work on chapter project
cd ch[XX]-[name]

# 3. Code and test
cargo watch -x test -x run

# 4. Document learnings
vim NOTES.md
```

---

## 📊 Progress Tracking

Update as you complete each chapter:

- [ ] Ch 1: Systems programming intro
- [ ] Ch 2: Tour of Rust
- [ ] Ch 3: Fundamental types
- [ ] Ch 4: Ownership
- [ ] Ch 5: References
- [ ] Ch 6: Expressions
- [ ] Ch 7: Error handling
- [ ] Ch 8: Crates and modules
- [ ] Ch 9: Structs
- [ ] Ch 10: Enums and patterns
- [ ] Ch 11: Traits and generics
- [ ] Ch 12: Operator overloading
- [ ] Ch 13: Utility traits
- [ ] Ch 14: Closures
- [ ] Ch 15: Iterators
- [ ] Ch 16: Collections
- [ ] Ch 17: Strings and text
- [ ] Ch 18: Input and output
- [ ] Ch 19: Concurrency
- [ ] Ch 20: Async programming
- [ ] Ch 21: Macros
- [ ] Ch 22: Unsafe code
- [ ] Ch 23: Foreign functions

---

## 🎯 Learning Goals

By completing this journey, you will:

✅ **Understand Rust fundamentals** - Ownership, borrowing, lifetimes
✅ **Master the type system** - Traits, generics, trait objects
✅ **Handle errors like a pro** - Result, Option, custom errors
✅ **Write concurrent code** - Threads, channels, async/await
✅ **Work with unsafe** - When and how to use unsafe Rust
✅ **Interface with C** - FFI for system programming
✅ **Build real programs** - 23 practical projects

---

## 📚 Resources

### Book
- **"Programming Rust" (2nd Edition, 2021)**
- Authors: Jim Blandy, Jason Orendorff, Leonora F.S. Tindall
- Publisher: O'Reilly Media
- ~700 pages

### Online
- [Rust Documentation](https://doc.rust-lang.org/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rust Book](https://doc.rust-lang.org/book/)

---

## 🤝 Related Repositories

| Repository | Book | Focus |
|------------|------|-------|
| **programming-rust-journey** (this repo) | Programming Rust | Comprehensive foundation |
| **[rustaceans-odyssey](https://github.com/Joshuaisikah/rustaceans-odyssey)** | Rust for Rustaceans | Advanced patterns |

---

## 📝 Documentation

- **START_HERE.md** - Getting started guide
- **CHAPTER_GUIDE.md** - Detailed chapter-by-chapter roadmap
- **[chapter]/README.md** - Specific chapter objectives
- **[chapter]/NOTES.md** - Your learning journal

---

## ✨ Features

- ✅ One project per chapter (23 total)
- ✅ Progressive difficulty
- ✅ Comprehensive coverage
- ✅ Learning journal templates
- ✅ Practice exercises
- ✅ Test-driven learning

---

**Start Date:** 2026-03-26

**Estimated Completion:** 3-6 months (1-2 chapters per week)

---

*"The best way to learn Rust is to write Rust."* 🦀

**Let's begin your journey!**
