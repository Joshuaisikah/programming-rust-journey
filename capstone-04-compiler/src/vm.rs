// vm.rs — Stack-based Virtual Machine
//
// Executes bytecode produced by codegen.
// Uses a value stack and a variable environment (HashMap).
// Supports integer arithmetic, boolean logic, variables,
// control flow (Jump/JumpIfFalse), and basic function calls.

use std::collections::HashMap;
use crate::codegen::{Bytecode, Instruction};
use crate::error::CompileError;

// ── Runtime values ────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Void,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(n)   => write!(f, "{}", n),
            Value::Float(x) => write!(f, "{}", x),
            Value::Bool(b)  => write!(f, "{}", b),
            Value::Str(s)   => write!(f, "{}", s),
            Value::Void     => write!(f, "void"),
        }
    }
}

// ── VM ────────────────────────────────────────────────────────

pub struct Vm {
    stack: Vec<Value>,
    vars: HashMap<String, Value>,
    ip: usize, // instruction pointer
}

impl Vm {
    pub fn new() -> Self {
        todo!("Vm {{ stack: vec![], vars: HashMap::new(), ip: 0 }}")
    }

    /// Execute bytecode and return the last integer value on the stack.
    /// Returns 0 if the stack is empty at the end.
    pub fn run(&mut self, code: Bytecode) -> Result<i64, CompileError> {
        todo!("loop over instructions, dispatch, handle control flow")
    }

    fn push(&mut self, v: Value) {
        todo!("self.stack.push(v)")
    }

    fn pop(&mut self) -> Result<Value, CompileError> {
        todo!("self.stack.pop().ok_or(RuntimeError(\"stack underflow\"))")
    }

    fn pop_int(&mut self) -> Result<i64, CompileError> {
        todo!("match self.pop()? {{ Value::Int(n) => Ok(n), v => Err(type error) }}")
    }

    fn pop_bool(&mut self) -> Result<bool, CompileError> {
        todo!("match self.pop()? {{ Value::Bool(b) => Ok(b), v => Err(type error) }}")
    }

    fn execute_binop_int<F>(&mut self, op: F) -> Result<(), CompileError>
    where
        F: Fn(i64, i64) -> Value,
    {
        todo!("pop two ints, push op(a, b)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compile_and_run;

    #[test]
    #[ignore = "implement vm"]
    fn test_integer_literal() {
        assert_eq!(compile_and_run("42;").unwrap(), 42);
    }

    #[test]
    #[ignore = "implement vm"]
    fn test_addition() {
        assert_eq!(compile_and_run("1 + 2;").unwrap(), 3);
    }

    #[test]
    #[ignore = "implement vm"]
    fn test_arithmetic_precedence() {
        assert_eq!(compile_and_run("2 + 3 * 4;").unwrap(), 14);
    }

    #[test]
    #[ignore = "implement vm"]
    fn test_let_and_variable_use() {
        assert_eq!(compile_and_run("let x = 10; x + 5;").unwrap(), 15);
    }

    #[test]
    #[ignore = "implement vm"]
    fn test_while_loop() {
        let src = "let n = 5; let acc = 0; while n > 0 { acc = acc + n; n = n - 1; } acc;";
        assert_eq!(compile_and_run(src).unwrap(), 15); // 5+4+3+2+1
    }

    #[test]
    #[ignore = "implement vm"]
    fn test_fibonacci_function() {
        let src = r#"
            fn fib(n: int) -> int {
                if n <= 1 { return n; }
                return fib(n - 1) + fib(n - 2);
            }
            fib(10);
        "#;
        assert_eq!(compile_and_run(src).unwrap(), 55);
    }

    #[test]
    #[ignore = "implement vm"]
    fn test_negation() {
        assert_eq!(compile_and_run("-5;").unwrap(), -5);
    }

    #[test]
    #[ignore = "implement vm"]
    fn test_boolean_and() {
        assert_eq!(compile_and_run("true && false;").unwrap(), 0); // false → 0
    }
}
