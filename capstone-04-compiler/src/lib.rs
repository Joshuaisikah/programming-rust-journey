// Capstone 4 — Mini Programming Language Compiler
//
// A complete pipeline for a small language called "Forge":
//   Lexer → Parser → AST → Type Checker → Bytecode → VM
//
// Forge supports: integers, booleans, strings, variables,
//   if/else, while loops, functions, and closures.

pub mod ast;
pub mod codegen;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod token;
pub mod typechecker;
pub mod vm;

pub use error::CompileError;
pub use vm::Vm;

/// Compile a source string and run it in the VM.
/// Returns the exit value of the program.
pub fn compile_and_run(source: &str) -> Result<i64, CompileError> {
    let tokens = lexer::tokenize(source)?;
    let ast    = parser::parse(tokens)?;
    let _      = typechecker::check(&ast)?;
    let bytecode = codegen::generate(&ast)?;
    let mut vm = Vm::new();
    vm.run(bytecode)
}
