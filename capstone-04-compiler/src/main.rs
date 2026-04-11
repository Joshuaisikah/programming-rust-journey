use forge_compiler::compile_and_run;

fn main() {
    println!("Forge Compiler — Capstone 4");
    println!();

    // Uncomment and implement the compiler pipeline to run these:
    //
    // let program = "
    //     fn fib(n: int) -> int {
    //         if n <= 1 { return n; }
    //         return fib(n - 1) + fib(n - 2);
    //     }
    //     fib(10);
    // ";
    //
    // match compile_and_run(program) {
    //     Ok(result) => println!("Result: {}", result),
    //     Err(e)     => eprintln!("Error: {}", e),
    // }

    println!("Implement src/ modules in order:");
    println!("  1. src/token.rs      — Token enum (done)");
    println!("  2. src/lexer.rs      — Tokenize source");
    println!("  3. src/ast.rs        — AST nodes (done)");
    println!("  4. src/parser.rs     — Recursive descent");
    println!("  5. src/typechecker.rs— Type checking");
    println!("  6. src/codegen.rs    — Bytecode generation");
    println!("  7. src/vm.rs         — Stack VM execution");
}
