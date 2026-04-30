# Capstone 04 — Mini Language Compiler (Forge)

## What You're Building

A complete compiler for a small programming language called **Forge**.
You write Forge source code, and your compiler runs it:

```
// example.forge
fn add(a, b) {
    return a + b;
}

let result = add(3, 4);
print(result);   // → 7
```

Forge supports: integers, booleans, strings, variables, if/else,
while loops, functions, and closures.

## Architecture: How the Files Connect

The compiler is a **pipeline** — each stage transforms the output of the previous one:

```
Source text (String)
     │
     ▼
  lexer.rs      tokenize(source) → Vec<Token>
     │              breaks text into meaningful chunks
     │              "let x = 5 + 3" → [Let, Ident("x"), Eq, Int(5), Plus, Int(3)]
     ▼
  token.rs      the Token enum — defines every possible token type
     │              shared by lexer.rs and parser.rs
     ▼
  parser.rs     parse(tokens) → AstNode (tree)
     │              turns a flat list of tokens into a nested tree structure
     │              "5 + 3 * 2" becomes Add(5, Mul(3, 2)) not Mul(Add(5,3), 2)
     ▼
  ast.rs        the AstNode enum — every possible node in the tree
     │              shared by parser.rs, typechecker.rs, and codegen.rs
     ▼
  typechecker.rs  check(ast) → Result<(), CompileError>
     │              walks the tree and verifies types are used correctly
     │              rejects "let x = 5 + true" before it reaches the VM
     ▼
  codegen.rs    generate(ast) → Vec<Instruction>
     │              translates the tree into a flat list of bytecode instructions
     │              the VM does not know about the tree, only instructions
     ▼
  vm.rs         run(bytecode) → i64
                   executes the instructions on a simple stack machine
                   returns the exit value of the program
```

`error.rs` sits outside the pipeline and is used by every stage.
`lib.rs` exposes `compile_and_run()` which runs the full pipeline in sequence.

## Why Each File Exists

**`main.rs`** — Reads a `.forge` file path from args, calls `compile_and_run()`,
prints the result or error. Thin entry point; nothing about the compiler lives here.

**`lib.rs`** — Exposes `compile_and_run(source)` which chains all stages in order.
This is the public API. The individual stages are also public so they can be tested
in isolation.

**`token.rs`** — The `Token` enum: every legal token in the Forge language
(`Let`, `Fn`, `If`, `Int(i64)`, `Ident(String)`, `Plus`, `LeftParen`, etc.).
Separated into its own file because both `lexer.rs` and `parser.rs` import it.
If it lived in `lexer.rs`, the parser would have to import the lexer just to get
the token type — a confusing dependency.

**`lexer.rs`** — `tokenize(source) → Vec<Token>`. Reads characters one at a time
and groups them into tokens. Separated because tokenization is a self-contained
problem. You can test it by asserting `tokenize("let x = 5") == [Let, Ident("x"), Eq, Int(5)]`
without involving any other stage.

**`ast.rs`** — The `AstNode` enum: every possible node in the syntax tree
(`BinaryOp`, `IfExpr`, `FnDef`, `FnCall`, `Block`, etc.). Separated for the same
reason as `token.rs` — three different stages (`parser`, `typechecker`, `codegen`)
all need this type and none of them should own it.

**`parser.rs`** — `parse(tokens) → AstNode`. Consumes the token list and builds the
tree. This is the most complex stage: it handles operator precedence, nested
expressions, and recursive structures. Separated because the parsing rules are a
complete sub-problem that can be unit tested in isolation.

**`typechecker.rs`** — `check(ast) → Result`. Walks the AST and verifies that types
are used correctly. Separated because type checking is logically distinct from
parsing. A program can be syntactically valid (it parses) but semantically wrong
(it type-checks). Running them as separate stages makes both easier to understand
and test.

**`codegen.rs`** — `generate(ast) → Vec<Instruction>`. Translates the tree into
bytecode. Separated from the VM so the instruction format is the only contract
between them. You can change how the VM executes instructions without touching
how they are generated.

**`vm.rs`** — `run(bytecode) → i64`. A simple stack machine: push values, execute
operations, call functions. Separated so it can be tested by feeding it raw
bytecode without running the full compiler pipeline.

**`error.rs`** — `CompileError` covers every failure across all stages: `LexError`,
`ParseError`, `TypeError`, `RuntimeError`. Separated so every stage returns
`Result<_, CompileError>` without importing from each other.

## Chapters It Combines

Ch10 Enums & Patterns · Ch15 Iterators · Ch21 Macros · Ch22 Unsafe (for the VM)

## Mental Model

Rust's own compiler (rustc) has the same pipeline: lexer → parser → AST →
type checker → MIR → codegen. After this capstone you will understand what
rustc is actually doing when it tells you "expected `i32`, found `&str`" —
because you wrote a type checker yourself.
