// lexer.rs — Tokenizer
//
// Converts source code string → Vec<Token>.
// Tracks line/column for error messages.

use crate::error::CompileError;
use crate::token::Token;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

/// Tokenize the source string into a list of spanned tokens.
/// The last token is always Token::Eof.
pub fn tokenize(source: &str) -> Result<Vec<Spanned<Token>>, CompileError> {
    todo!("scan characters, produce tokens with spans")
}

// ── Lexer struct (internal) ───────────────────────────────────

struct Lexer<'a> {
    source: &'a str,
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        todo!("Lexer {{ source, chars: source.char_indices().peekable(), line: 1, col: 1 }}")
    }

    fn current_span(&self) -> Span {
        todo!("Span {{ line: self.line, col: self.col }}")
    }

    fn advance(&mut self) -> Option<char> {
        todo!("pop next char, update line/col tracking")
    }

    fn peek(&mut self) -> Option<char> {
        todo!("self.chars.peek().map(|(_, c)| *c)")
    }

    fn skip_whitespace(&mut self) {
        todo!("advance while peek is whitespace")
    }

    fn skip_line_comment(&mut self) {
        todo!("advance until newline or EOF")
    }

    fn read_number(&mut self, first: char) -> Result<Token, CompileError> {
        todo!("collect digits, optional decimal point, parse to Integer or Float")
    }

    fn read_string(&mut self) -> Result<Token, CompileError> {
        todo!("collect chars until closing quote, handle escape sequences")
    }

    fn read_ident_or_keyword(&mut self, first: char) -> Token {
        todo!("collect alphanumeric/_chars, check keyword table, return Ident or keyword")
    }

    fn next_token(&mut self) -> Result<Spanned<Token>, CompileError> {
        todo!("main dispatch: skip whitespace/comments, match first char, call appropriate reader")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokens(src: &str) -> Vec<Token> {
        tokenize(src).unwrap().into_iter().map(|s| s.node).collect()
    }

    #[test]
    #[ignore = "implement tokenize"]
    fn test_tokenize_empty_source() {
        let t = tokens("");
        assert_eq!(t, vec![Token::Eof]);
    }

    #[test]
    #[ignore = "implement tokenize"]
    fn test_tokenize_integer() {
        let t = tokens("42");
        assert_eq!(t, vec![Token::Integer(42), Token::Eof]);
    }

    #[test]
    #[ignore = "implement tokenize"]
    fn test_tokenize_string_literal() {
        let t = tokens("\"hello\"");
        assert_eq!(t, vec![Token::StringLit("hello".into()), Token::Eof]);
    }

    #[test]
    #[ignore = "implement tokenize"]
    fn test_tokenize_keywords() {
        let t = tokens("let fn if else while return");
        assert_eq!(t, vec![
            Token::Let, Token::Fn, Token::If, Token::Else,
            Token::While, Token::Return, Token::Eof,
        ]);
    }

    #[test]
    #[ignore = "implement tokenize"]
    fn test_tokenize_operators() {
        let t = tokens("+ - * / == != < <= > >= && ||");
        assert_eq!(t, vec![
            Token::Plus, Token::Minus, Token::Star, Token::Slash,
            Token::EqEq, Token::BangEq,
            Token::Lt, Token::LtEq, Token::Gt, Token::GtEq,
            Token::AmpAmp, Token::PipePipe,
            Token::Eof,
        ]);
    }

    #[test]
    #[ignore = "implement tokenize"]
    fn test_tokenize_line_comment_ignored() {
        let t = tokens("42 // this is a comment\n99");
        assert_eq!(t, vec![Token::Integer(42), Token::Integer(99), Token::Eof]);
    }

    #[test]
    #[ignore = "implement tokenize"]
    fn test_tokenize_booleans() {
        let t = tokens("true false");
        assert_eq!(t, vec![Token::Bool(true), Token::Bool(false), Token::Eof]);
    }

    #[test]
    #[ignore = "implement tokenize"]
    fn test_tokenize_spans_track_line() {
        let spanned = tokenize("let\nx").unwrap();
        assert_eq!(spanned[0].span.line, 1);
        assert_eq!(spanned[1].span.line, 2); // 'x' is on line 2
    }
}
