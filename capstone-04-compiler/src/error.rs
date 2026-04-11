// error.rs — Compiler error types

use thiserror::Error;
use crate::lexer::Span;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("Lexer error at {span:?}: {message}")]
    LexError { span: Span, message: String },

    #[error("Parse error at {span:?}: {message}")]
    ParseError { span: Span, message: String },

    #[error("Type error at {span:?}: {message}")]
    TypeError { span: Span, message: String },

    #[error("Undefined variable '{name}' at {span:?}")]
    UndefinedVariable { name: String, span: Span },

    #[error("Undefined function '{name}' at {span:?}")]
    UndefinedFunction { name: String, span: Span },

    #[error("Runtime error: {0}")]
    RuntimeError(String),
}

impl CompileError {
    pub fn lex(span: Span, message: impl Into<String>) -> Self {
        CompileError::LexError { span, message: message.into() }
    }

    pub fn parse(span: Span, message: impl Into<String>) -> Self {
        CompileError::ParseError { span, message: message.into() }
    }

    pub fn type_error(span: Span, message: impl Into<String>) -> Self {
        CompileError::TypeError { span, message: message.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span() -> Span { Span { line: 5, col: 10 } }

    #[test]
    fn test_lex_error_contains_message() {
        let e = CompileError::lex(span(), "unexpected character '@'");
        assert!(e.to_string().contains("unexpected character"));
    }

    #[test]
    fn test_parse_error_contains_span() {
        let e = CompileError::parse(span(), "expected ')'");
        let s = e.to_string();
        assert!(s.contains("5")); // line number
    }

    #[test]
    fn test_undefined_variable() {
        let e = CompileError::UndefinedVariable { name: "foo".into(), span: span() };
        assert!(e.to_string().contains("foo"));
    }
}
