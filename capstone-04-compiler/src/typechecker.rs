// typechecker.rs — Static type checker
//
// Walks the AST and verifies type correctness:
//   - All variables are declared before use
//   - Binary operators applied to compatible types
//   - Function calls match declared signatures
//   - Return types match function declarations

use std::collections::HashMap;
use crate::ast::{BinOp, Expr, Program, Stmt, TypeAnnotation, UnaryOp};
use crate::error::CompileError;
use crate::lexer::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Str,
    Void,
    Fn { params: Vec<Type>, ret: Box<Type> },
}

impl Type {
    pub fn from_annotation(ann: &TypeAnnotation) -> Self {
        match ann {
            TypeAnnotation::Int   => Type::Int,
            TypeAnnotation::Float => Type::Float,
            TypeAnnotation::Bool  => Type::Bool,
            TypeAnnotation::Str   => Type::Str,
            TypeAnnotation::Void  => Type::Void,
            TypeAnnotation::Named(n) => todo!("look up named type {}", n),
        }
    }
}

pub struct TypeChecker {
    /// Stack of scopes: each scope maps variable names to their types.
    scopes: Vec<HashMap<String, Type>>,
    /// Current function's expected return type (None if at top level).
    return_type: Option<Type>,
}

/// Type-check a program. Returns Ok(()) if no type errors.
pub fn check(program: &Program) -> Result<(), CompileError> {
    let mut tc = TypeChecker::new();
    tc.check_program(program)
}

impl TypeChecker {
    pub fn new() -> Self {
        todo!("TypeChecker {{ scopes: vec![HashMap::new()], return_type: None }}")
    }

    fn push_scope(&mut self) {
        todo!("self.scopes.push(HashMap::new())")
    }

    fn pop_scope(&mut self) {
        todo!("self.scopes.pop()")
    }

    fn define(&mut self, name: &str, ty: Type) {
        todo!("insert into innermost scope")
    }

    fn lookup(&self, name: &str, span: Span) -> Result<Type, CompileError> {
        todo!("search scopes from innermost outward, UndefinedVariable if not found")
    }

    fn check_program(&mut self, program: &Program) -> Result<(), CompileError> {
        todo!("check each statement")
    }

    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), CompileError> {
        todo!("dispatch on stmt variant")
    }

    fn check_expr(&mut self, expr: &Expr) -> Result<Type, CompileError> {
        todo!("return the type of the expression, or TypeError")
    }

    fn check_binop(&self, op: &BinOp, left: &Type, right: &Type, span: Span) -> Result<Type, CompileError> {
        todo!("verify operand types are compatible with op, return result type")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::parser::parse;

    fn check_src(src: &str) -> Result<(), CompileError> {
        let tokens = tokenize(src)?;
        let ast = parse(tokens)?;
        check(&ast)
    }

    #[test]
    #[ignore = "implement typechecker"]
    fn test_valid_let_and_use() {
        assert!(check_src("let x = 42; x + 1;").is_ok());
    }

    #[test]
    #[ignore = "implement typechecker"]
    fn test_undefined_variable_errors() {
        assert!(check_src("x + 1;").is_err());
    }

    #[test]
    #[ignore = "implement typechecker"]
    fn test_type_mismatch_in_binop() {
        // Can't add a bool and an integer
        assert!(check_src("true + 1;").is_err());
    }

    #[test]
    #[ignore = "implement typechecker"]
    fn test_valid_comparison() {
        assert!(check_src("let x = 5; x > 3;").is_ok());
    }

    #[test]
    #[ignore = "implement typechecker"]
    fn test_function_return_type_mismatch_errors() {
        assert!(check_src("fn f() -> int { return true; }").is_err());
    }

    #[test]
    #[ignore = "implement typechecker"]
    fn test_scoping_inner_var_not_visible_outside() {
        assert!(check_src("while true { let inner = 1; } inner;").is_err());
    }
}
