// capstone-04 — Forge Compiler: Mini Programming Language
//
// ── THE PIPELINE (source text → result) ──────────────────────
//
//   source: &str
//       ↓
//   lexer::tokenize(source)
//       "let x = 10;"  →  [Let, Ident("x"), Eq, Integer(10), Semicolon, Eof]
//       Each token carries a Span (line, col) for error messages.
//       ↓
//   parser::parse(tokens)
//       tokens → AST (Abstract Syntax Tree)
//       e.g. Let { name: "x", value: Expr::Int(10) }
//       Uses recursive descent: parse_statement, parse_expr, parse_block, etc.
//       ↓
//   typechecker::check(&ast)
//       walks the AST, ensures types are consistent (no int + bool, etc.)
//       tracks variable scopes, checks function signatures
//       ↓
//   codegen::generate(&ast)
//       walks the AST, emits a Vec<Instruction> (Bytecode)
//       e.g. Push(10), Store("x"), Load("x"), Push(5), Add
//       ↓
//   vm::Vm::new().run(bytecode)
//       executes instructions on a value stack
//       returns the last integer left on the stack
//
//   lib.rs compile_and_run(source) wraps the whole pipeline in one call.
//
// ── IMPLEMENTATION ORDER ──────────────────────────────────────
//
//   1. token.rs       — Token::fmt (Display for error messages)
//                       Everything else already done: variants, is_literal,
//                       is_binary_op, precedence. Just implement fmt.
//
//   2. error.rs       — CompileError variants: LexError, ParseError,
//                       TypeError, RuntimeError. Add as you need them.
//
//   3. lexer.rs       — Lexer::new, advance, peek, skip_whitespace,
//                       read_number, read_string, read_ident_or_keyword,
//                       next_token, then tokenize(source)
//                       Test: tokenize("let x = 42;") → [Let, Ident, Eq, Int, Semi, Eof]
//
//   4. ast.rs         — Already defined. Read it to understand the tree shape
//                       before writing the parser.
//
//   5. parser.rs      — parse(tokens) → Ast
//                       parse_statement: let, fn, if, while, return, expr-stmt
//                       parse_expr with Pratt parsing (use Token::precedence)
//                       parse_block: { statement* }
//
//   6. typechecker.rs — check(&ast) → Result<(), CompileError>
//                       Build a symbol table, check that variables are declared
//                       before use, function calls match signatures.
//
//   7. codegen.rs     — generate(&ast) → Result<Bytecode, CompileError>
//                       Emit: Push, Pop, Add/Sub/Mul/Div, Store/Load,
//                       Jump, JumpIfFalse (for if/while), Call/Return.
//
//   8. vm.rs          — Vm::new, run(bytecode)
//                       Stack-based: push/pop, arithmetic, store/load vars,
//                       jump for control flow, call stack for functions.
//
// ── ONCE COMPLETE ─────────────────────────────────────────────

use forge_compiler::compile_and_run;

fn main() {
    println!("Forge Compiler — Capstone 4");
    println!();
    println!("Run `cargo test -p capstone-04-compiler` to track progress.");

    // Uncomment once implemented — these programs should all work:
    //
    // // Basic arithmetic
    // println!("{:?}", compile_and_run("2 + 3 * 4;"));  // Ok(14)
    //
    // // Variables
    // println!("{:?}", compile_and_run("let x = 10; x + 5;"));  // Ok(15)
    //
    // // If / else
    // let prog = "let x = 5; if x > 3 { x * 2; } else { x; }";
    // println!("{:?}", compile_and_run(prog));  // Ok(10)
    //
    // // While loop (sum 1..5)
    // let prog = "let n = 5; let acc = 0;
    //             while n > 0 { acc = acc + n; n = n - 1; } acc;";
    // println!("{:?}", compile_and_run(prog));  // Ok(15)
    //
    // // Recursive function
    // let prog = r#"
    //     fn fib(n: int) -> int {
    //         if n <= 1 { return n; }
    //         return fib(n - 1) + fib(n - 2);
    //     }
    //     fib(10);
    // "#;
    // println!("{:?}", compile_and_run(prog));  // Ok(55)
}
