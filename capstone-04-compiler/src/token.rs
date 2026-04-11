// token.rs — Token types produced by the lexer
//
// The Forge language has:
//   integers, booleans, strings, identifiers
//   arithmetic: + - * / %
//   comparison: == != < <= > >=
//   logic: && || !
//   assignment: =
//   keywords: let, fn, if, else, while, return, true, false
//   delimiters: ( ) { } , ; :

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Integer(i64),
    Float(f64),
    Bool(bool),
    StringLit(String),

    // Identifier or keyword
    Ident(String),

    // Keywords
    Let,
    Fn,
    If,
    Else,
    While,
    Return,

    // Arithmetic
    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    // Comparison
    EqEq,
    BangEq,
    Lt,
    LtEq,
    Gt,
    GtEq,

    // Logic
    AmpAmp,
    PipePipe,
    Bang,

    // Assignment
    Eq,

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Colon,
    Arrow, // ->

    // Special
    Eof,
}

impl Token {
    /// Return true if this token is a literal value (integer, float, bool, string).
    pub fn is_literal(&self) -> bool {
        matches!(self, Token::Integer(_) | Token::Float(_) | Token::Bool(_) | Token::StringLit(_))
    }

    /// Return true if this token represents a binary operator.
    pub fn is_binary_op(&self) -> bool {
        matches!(
            self,
            Token::Plus | Token::Minus | Token::Star | Token::Slash |
            Token::Percent | Token::EqEq | Token::BangEq |
            Token::Lt | Token::LtEq | Token::Gt | Token::GtEq |
            Token::AmpAmp | Token::PipePipe
        )
    }

    /// Operator precedence (higher = tighter binding). Returns None for non-operators.
    pub fn precedence(&self) -> Option<u8> {
        match self {
            Token::PipePipe                              => Some(1),
            Token::AmpAmp                               => Some(2),
            Token::EqEq | Token::BangEq                => Some(3),
            Token::Lt | Token::LtEq | Token::Gt | Token::GtEq => Some(4),
            Token::Plus | Token::Minus                  => Some(5),
            Token::Star | Token::Slash | Token::Percent => Some(6),
            _                                           => None,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("write human-readable token name for error messages")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_literal_for_literals() {
        assert!(Token::Integer(42).is_literal());
        assert!(Token::Bool(true).is_literal());
        assert!(Token::StringLit("hi".into()).is_literal());
    }

    #[test]
    fn test_is_literal_false_for_non_literals() {
        assert!(!Token::Plus.is_literal());
        assert!(!Token::Ident("x".into()).is_literal());
    }

    #[test]
    fn test_is_binary_op() {
        assert!(Token::Plus.is_binary_op());
        assert!(Token::EqEq.is_binary_op());
        assert!(!Token::Bang.is_binary_op()); // unary
        assert!(!Token::Integer(1).is_binary_op());
    }

    #[test]
    fn test_precedence_ordering() {
        // Multiplication binds tighter than addition
        assert!(Token::Star.precedence() > Token::Plus.precedence());
        // Logical AND binds tighter than OR
        assert!(Token::AmpAmp.precedence() > Token::PipePipe.precedence());
        // Non-operators have no precedence
        assert_eq!(Token::Integer(1).precedence(), None);
    }
}
