// codegen.rs — Bytecode generator
//
// Walks the type-checked AST and emits a flat list of Bytecode instructions
// for the VM to execute.

use crate::ast::{BinOp, Expr, Program, Stmt, UnaryOp};
use crate::error::CompileError;

// ── Bytecode instruction set ──────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Stack manipulation
    PushInt(i64),
    PushFloat(f64),
    PushBool(bool),
    PushStr(String),
    Pop,
    Dup,

    // Variables (by name for now; a real compiler would use indices)
    LoadVar(String),
    StoreVar(String),

    // Arithmetic
    Add, Sub, Mul, Div, Mod,
    Neg,

    // Comparison → pushes Bool
    Eq, Ne, Lt, Le, Gt, Ge,

    // Logic
    And, Or, Not,

    // Control flow
    Jump(usize),        // unconditional jump to instruction index
    JumpIfFalse(usize), // pop bool, jump if false
    Call { name: String, arity: usize },
    Return,

    // I/O (built-ins)
    Print,
    PrintLn,

    // Function definition marker
    FnStart { name: String, arity: usize, end: usize },
    FnEnd,
}

pub type Bytecode = Vec<Instruction>;

/// Compile a type-checked AST into bytecode.
pub fn generate(program: &Program) -> Result<Bytecode, CompileError> {
    let mut gen = CodeGen::new();
    gen.generate_program(program)?;
    Ok(gen.instructions)
}

struct CodeGen {
    instructions: Bytecode,
}

impl CodeGen {
    fn new() -> Self {
        todo!("CodeGen {{ instructions: vec![] }}")
    }

    fn emit(&mut self, instr: Instruction) {
        todo!("self.instructions.push(instr)")
    }

    /// Emit a placeholder Jump and return its index, to be patched later.
    fn emit_jump_placeholder(&mut self) -> usize {
        todo!("push Jump(0), return index of that instruction")
    }

    /// Patch a previously emitted jump to point to the current instruction.
    fn patch_jump(&mut self, idx: usize) {
        todo!("self.instructions[idx] = Jump(self.instructions.len())")
    }

    fn generate_program(&mut self, program: &Program) -> Result<(), CompileError> {
        todo!("generate code for each statement")
    }

    fn generate_stmt(&mut self, stmt: &Stmt) -> Result<(), CompileError> {
        todo!("dispatch on stmt variant")
    }

    fn generate_expr(&mut self, expr: &Expr) -> Result<(), CompileError> {
        todo!("emit instructions that leave the expr value on the stack")
    }

    fn binop_instruction(op: &BinOp) -> Instruction {
        match op {
            BinOp::Add => Instruction::Add,
            BinOp::Sub => Instruction::Sub,
            BinOp::Mul => Instruction::Mul,
            BinOp::Div => Instruction::Div,
            BinOp::Mod => Instruction::Mod,
            BinOp::Eq  => Instruction::Eq,
            BinOp::Ne  => Instruction::Ne,
            BinOp::Lt  => Instruction::Lt,
            BinOp::Le  => Instruction::Le,
            BinOp::Gt  => Instruction::Gt,
            BinOp::Ge  => Instruction::Ge,
            BinOp::And => Instruction::And,
            BinOp::Or  => Instruction::Or,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::parser::parse;

    fn gen_src(src: &str) -> Result<Bytecode, CompileError> {
        let tokens = tokenize(src)?;
        let ast = parse(tokens)?;
        generate(&ast)
    }

    #[test]
    #[ignore = "implement codegen"]
    fn test_integer_literal_emits_push() {
        let bc = gen_src("42;").unwrap();
        assert!(bc.contains(&Instruction::PushInt(42)));
    }

    #[test]
    #[ignore = "implement codegen"]
    fn test_addition_emits_add() {
        let bc = gen_src("1 + 2;").unwrap();
        assert!(bc.contains(&Instruction::PushInt(1)));
        assert!(bc.contains(&Instruction::PushInt(2)));
        assert!(bc.contains(&Instruction::Add));
    }

    #[test]
    #[ignore = "implement codegen"]
    fn test_let_emits_store() {
        let bc = gen_src("let x = 7;").unwrap();
        assert!(bc.contains(&Instruction::PushInt(7)));
        assert!(bc.contains(&Instruction::StoreVar("x".to_string())));
    }

    #[test]
    #[ignore = "implement codegen"]
    fn test_variable_load() {
        let bc = gen_src("let x = 7; x;").unwrap();
        assert!(bc.contains(&Instruction::LoadVar("x".to_string())));
    }
}
