// parser.rs — Recursive descent parser
//
// Converts Vec<Spanned<Token>> → Program (Vec<Stmt>).
// Uses Pratt (top-down operator precedence) parsing for expressions.

use crate::ast::{BinOp, Expr, FnDef, Param, Program, Stmt, TypeAnnotation, UnaryOp};
use crate::error::CompileError;
use crate::lexer::Spanned;
use crate::token::Token;

/// Parse a token stream into a Program (top-level statement list).
pub fn parse(tokens: Vec<Spanned<Token>>) -> Result<Program, CompileError> {
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}

struct Parser {
    tokens: Vec<Spanned<Token>>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Spanned<Token>>) -> Self {
        todo!("Parser {{ tokens, pos: 0 }}")
    }

    fn peek(&self) -> &Token {
        todo!("&self.tokens[self.pos].node, or &Token::Eof if out of bounds")
    }

    fn advance(&mut self) -> &Spanned<Token> {
        todo!("return current token, increment pos")
    }

    fn expect(&mut self, expected: Token) -> Result<&Spanned<Token>, CompileError> {
        todo!("advance if peek == expected, else return ParseError")
    }

    fn parse_program(&mut self) -> Result<Program, CompileError> {
        todo!("loop: parse statements until Eof, collect into Vec")
    }

    fn parse_stmt(&mut self) -> Result<Stmt, CompileError> {
        todo!("dispatch on peek: Let → parse_let, Fn → parse_fn, While → parse_while, else → parse_expr_stmt")
    }

    fn parse_let(&mut self) -> Result<Stmt, CompileError> {
        todo!("consume Let, ident, Eq, expr, Semicolon; return Stmt::Let")
    }

    fn parse_while(&mut self) -> Result<Stmt, CompileError> {
        todo!("consume While, parse cond expr, parse block body")
    }

    fn parse_fn(&mut self) -> Result<Stmt, CompileError> {
        todo!("consume Fn, ident, params, optional return type, block body")
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, CompileError> {
        todo!("consume LBrace, parse stmts until RBrace, consume RBrace")
    }

    fn parse_expr(&mut self, min_prec: u8) -> Result<Expr, CompileError> {
        todo!("Pratt parser: parse left, then loop while next op has higher prec")
    }

    fn parse_prefix(&mut self) -> Result<Expr, CompileError> {
        todo!("literals, idents, unary ops, parenthesized exprs, if, fn calls")
    }

    fn parse_call_args(&mut self) -> Result<Vec<Expr>, CompileError> {
        todo!("consume LParen, comma-separated exprs, RParen")
    }

    fn parse_params(&mut self) -> Result<Vec<Param>, CompileError> {
        todo!("LParen, name:type pairs, RParen")
    }

    fn parse_type(&mut self) -> Result<TypeAnnotation, CompileError> {
        todo!("match ident → int/float/bool/str/void or Named")
    }

    fn token_to_binop(tok: &Token) -> Option<BinOp> {
        match tok {
            Token::Plus     => Some(BinOp::Add),
            Token::Minus    => Some(BinOp::Sub),
            Token::Star     => Some(BinOp::Mul),
            Token::Slash    => Some(BinOp::Div),
            Token::Percent  => Some(BinOp::Mod),
            Token::EqEq     => Some(BinOp::Eq),
            Token::BangEq   => Some(BinOp::Ne),
            Token::Lt       => Some(BinOp::Lt),
            Token::LtEq     => Some(BinOp::Le),
            Token::Gt       => Some(BinOp::Gt),
            Token::GtEq     => Some(BinOp::Ge),
            Token::AmpAmp   => Some(BinOp::And),
            Token::PipePipe => Some(BinOp::Or),
            _               => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    fn parse_src(src: &str) -> Result<Program, CompileError> {
        let tokens = tokenize(src)?;
        parse(tokens)
    }

    #[test]
    #[ignore = "implement parser"]
    fn test_parse_let_statement() {
        let prog = parse_src("let x = 42;").unwrap();
        assert_eq!(prog.len(), 1);
        assert!(matches!(prog[0], Stmt::Let { .. }));
    }

    #[test]
    #[ignore = "implement parser"]
    fn test_parse_integer_literal() {
        let prog = parse_src("42;").unwrap();
        assert!(matches!(prog[0], Stmt::Expr(Expr::Integer(42))));
    }

    #[test]
    #[ignore = "implement parser"]
    fn test_parse_addition() {
        let prog = parse_src("1 + 2;").unwrap();
        if let Stmt::Expr(Expr::BinOp { op: BinOp::Add, .. }) = &prog[0] {
        } else {
            panic!("expected Add BinOp");
        }
    }

    #[test]
    #[ignore = "implement parser"]
    fn test_parse_operator_precedence() {
        // 1 + 2 * 3 should parse as 1 + (2 * 3)
        let prog = parse_src("1 + 2 * 3;").unwrap();
        if let Stmt::Expr(Expr::BinOp { op: BinOp::Add, right, .. }) = &prog[0] {
            assert!(matches!(right.as_ref(), Expr::BinOp { op: BinOp::Mul, .. }));
        } else {
            panic!("expected Add at top level");
        }
    }

    #[test]
    #[ignore = "implement parser"]
    fn test_parse_if_expression() {
        let prog = parse_src("if true { 1; }").unwrap();
        assert!(matches!(prog[0], Stmt::Expr(Expr::If { .. })));
    }

    #[test]
    #[ignore = "implement parser"]
    fn test_parse_function_def() {
        let prog = parse_src("fn add(a: int, b: int) -> int { return a + b; }").unwrap();
        assert!(matches!(prog[0], Stmt::FnDef(_)));
    }

    #[test]
    #[ignore = "implement parser"]
    fn test_parse_while_loop() {
        let prog = parse_src("while x > 0 { x = x - 1; }").unwrap();
        assert!(matches!(prog[0], Stmt::While { .. }));
    }

    #[test]
    #[ignore = "implement parser"]
    fn test_parse_function_call() {
        let prog = parse_src("print(42);").unwrap();
        if let Stmt::Expr(Expr::Call { args, .. }) = &prog[0] {
            assert_eq!(args.len(), 1);
        } else {
            panic!("expected Call");
        }
    }
}
