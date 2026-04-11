// request.rs — HTTP/1.1 request parsing
//
// Parses the raw bytes from a TCP connection into a structured Request.
// Handles: request line, headers, optional body.
// Does NOT use any HTTP library — hand-rolled parser.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
}

impl Method {
    /// Parse an HTTP method string (case-insensitive).
    pub fn parse(s: &str) -> Option<Self> {
        todo!("match s.to_uppercase().as_str()")
    }

    pub fn as_str(&self) -> &'static str {
        todo!("match self → \"GET\" | \"POST\" | ...")
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    /// The path portion of the URL (no query string).
    pub path: String,
    /// Parsed query parameters from the URL.
    pub query: HashMap<String, String>,
    /// Request headers (lowercase keys).
    pub headers: HashMap<String, String>,
    /// Raw request body, if any.
    pub body: Vec<u8>,
    /// Path parameters extracted by the router (e.g., `:id`).
    pub params: HashMap<String, String>,
}

impl Request {
    /// Parse a raw HTTP request from bytes.
    /// Returns Err if the request line or headers are malformed.
    pub fn parse(raw: &[u8]) -> Result<Self, crate::error::HttpError> {
        todo!("parse request line (method, path, version), then headers, then body")
    }

    /// Return the value of a header by name (case-insensitive lookup).
    pub fn header(&self, name: &str) -> Option<&str> {
        todo!("self.headers.get(&name.to_lowercase()).map(|s| s.as_str())")
    }

    /// Return Content-Length header as usize, or 0 if absent.
    pub fn content_length(&self) -> usize {
        todo!("parse Content-Length header as usize")
    }

    /// Return the body as a UTF-8 string, if valid.
    pub fn body_str(&self) -> Option<&str> {
        todo!("std::str::from_utf8(&self.body).ok()")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Method::parse ─────────────────────────────────────────

    #[test]
    #[ignore = "implement Method::parse"]
    fn test_parse_get() {
        assert_eq!(Method::parse("GET"), Some(Method::Get));
        assert_eq!(Method::parse("get"), Some(Method::Get));
    }

    #[test]
    #[ignore = "implement Method::parse"]
    fn test_parse_post() {
        assert_eq!(Method::parse("POST"), Some(Method::Post));
    }

    #[test]
    #[ignore = "implement Method::parse"]
    fn test_parse_unknown_returns_none() {
        assert_eq!(Method::parse("BREW"), None);
    }

    // ── Request::parse ────────────────────────────────────────

    fn raw(s: &str) -> Vec<u8> { s.as_bytes().to_vec() }

    #[test]
    #[ignore = "implement Request::parse"]
    fn test_parse_simple_get() {
        let r = Request::parse(&raw("GET /index.html HTTP/1.1\r\nHost: localhost\r\n\r\n")).unwrap();
        assert_eq!(r.method, Method::Get);
        assert_eq!(r.path, "/index.html");
        assert_eq!(r.header("host"), Some("localhost"));
    }

    #[test]
    #[ignore = "implement Request::parse"]
    fn test_parse_path_with_query() {
        let r = Request::parse(&raw("GET /search?q=rust&page=2 HTTP/1.1\r\n\r\n")).unwrap();
        assert_eq!(r.path, "/search");
        assert_eq!(r.query.get("q"), Some(&"rust".to_string()));
        assert_eq!(r.query.get("page"), Some(&"2".to_string()));
    }

    #[test]
    #[ignore = "implement Request::parse"]
    fn test_parse_post_with_body() {
        let r = Request::parse(&raw(
            "POST /users HTTP/1.1\r\nContent-Length: 13\r\n\r\n{\"name\":\"Bob\"}"
        )).unwrap();
        assert_eq!(r.method, Method::Post);
        assert_eq!(r.content_length(), 13);
        assert_eq!(r.body_str(), Some("{\"name\":\"Bob\"}"));
    }

    #[test]
    #[ignore = "implement Request::parse"]
    fn test_parse_malformed_returns_error() {
        assert!(Request::parse(&raw("not a request")).is_err());
    }
}
