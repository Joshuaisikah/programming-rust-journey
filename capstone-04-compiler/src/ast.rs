// ast.rs — Abstract Syntax Tree node definitions
//
// The AST represents a parsed Forge program.
// Every node is an enum or struct; enums use Box<> for recursive types.

use crate::lexer::Span;

/// A complete program is a list of statements.
pub type Program = Vec<Stmt>;

// ── Statements ────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Stmt {
    /// `let x = expr;`
    Let { name: String, value: Expr },
    /// `return expr;` or `return;`
    Return(Option<Expr>),
    /// An expression used as a statement (e.g., a function call).
    Expr(Expr),
    /// `while cond { body }`
    While { cond: Expr, body: Vec<Stmt> },
    /// `fn name(params) -> return_type { body }`
    FnDef(FnDef),
}

// ── Expressions ───────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    Bool(bool),
    StringLit(String),
    Ident(String),

    /// Binary operation: `left op right`
    BinOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
        span: Span,
    },

    /// Unary operation: `op expr`
    UnaryOp {
        op: UnaryOp,
        operand: Box<Expr>,
        span: Span,
    },

    /// `if cond { then } else { else_ }`
    If {
        cond: Box<Expr>,
        then: Vec<Stmt>,
        else_: Option<Vec<Stmt>>,
        span: Span,
    },

    /// Function call: `name(args)`
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        span: Span,
    },

    /// Assignment: `name = value`
    Assign {
        name: String,
        value: Box<Expr>,
        span: Span,
    },
}

// ── Operators ─────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,  // -x
    Not,  // !x
}

// ── Function definition ───────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: TypeAnnotation,
}

#[derive(Debug, Clone)]
pub enum TypeAnnotation {
    Int,
    Float,
    Bool,
    Str,
    Void,
    Named(String),
}

#[derive(Debug, Clone)]
pub struct FnDef {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: TypeAnnotation,
    pub body: Vec<Stmt>,
    pub span: Span,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Span;

    fn dummy_span() -> Span { Span { line: 0, col: 0 } }

    #[test]
    fn test_binop_can_be_constructed() {
        let e = Expr::BinOp {
            op: BinOp::Add,
            left: Box::new(Expr::Integer(1)),
            right: Box::new(Expr::Integer(2)),
            span: dummy_span(),
        };
        assert!(matches!(e, Expr::BinOp { op: BinOp::Add, .. }));
    }

    #[test]
    fn test_unary_neg() {
        let e = Expr::UnaryOp {
            op: UnaryOp::Neg,
            operand: Box::new(Expr::Integer(5)),
            span: dummy_span(),
        };
        assert!(matches!(e, Expr::UnaryOp { op: UnaryOp::Neg, .. }));
    }

    #[test]
    fn test_call_stores_args() {
        let e = Expr::Call {
            callee: Box::new(Expr::Ident("print".into())),
            args: vec![Expr::Integer(42)],
            span: dummy_span(),
        };
        if let Expr::Call { args, .. } = e {
            assert_eq!(args.len(), 1);
        } else {
            panic!("expected Call");
        }
    }
}
