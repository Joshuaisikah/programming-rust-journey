// response.rs — HTTP/1.1 response builder

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum StatusCode {
    Ok = 200,
    Created = 201,
    NoContent = 204,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    InternalServerError = 500,
}

impl StatusCode {
    pub fn as_u16(self) -> u16 { self as u16 }

    pub fn reason_phrase(self) -> &'static str {
        todo!("match self → \"OK\" | \"Created\" | ...")
    }
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    /// Create a response with the given status and empty body.
    pub fn new(status: StatusCode) -> Self {
        todo!("Response {{ status, headers: HashMap::new(), body: vec![] }}")
    }

    /// 200 OK with a plain-text body.
    pub fn ok(body: impl Into<String>) -> Self {
        todo!("Response::new(StatusCode::Ok) with Content-Type: text/plain")
    }

    /// 200 OK with a JSON body.
    pub fn json(body: impl Into<String>) -> Self {
        todo!("Response::new(StatusCode::Ok) with Content-Type: application/json")
    }

    /// 404 Not Found.
    pub fn not_found() -> Self {
        todo!("Response::new(StatusCode::NotFound)")
    }

    /// 500 Internal Server Error.
    pub fn server_error(msg: impl Into<String>) -> Self {
        todo!("Response::new(StatusCode::InternalServerError) with msg as body")
    }

    /// Add or overwrite a response header.
    pub fn header(mut self, name: &str, value: &str) -> Self {
        todo!("self.headers.insert(name.to_lowercase(), value.to_string()); self")
    }

    /// Serialize this response to HTTP/1.1 wire format bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        todo!("status line + headers + blank line + body")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_code_values() {
        assert_eq!(StatusCode::Ok.as_u16(), 200);
        assert_eq!(StatusCode::NotFound.as_u16(), 404);
        assert_eq!(StatusCode::InternalServerError.as_u16(), 500);
    }

    #[test]
    #[ignore = "implement StatusCode::reason_phrase"]
    fn test_reason_phrases() {
        assert_eq!(StatusCode::Ok.reason_phrase(), "OK");
        assert_eq!(StatusCode::NotFound.reason_phrase(), "Not Found");
        assert_eq!(StatusCode::Created.reason_phrase(), "Created");
    }

    #[test]
    #[ignore = "implement Response::ok"]
    fn test_ok_response_status() {
        let r = Response::ok("hello");
        assert_eq!(r.status, StatusCode::Ok);
    }

    #[test]
    #[ignore = "implement Response::to_bytes"]
    fn test_to_bytes_starts_with_status_line() {
        let r = Response::ok("hi");
        let bytes = r.to_bytes();
        let s = std::str::from_utf8(&bytes).unwrap();
        assert!(s.starts_with("HTTP/1.1 200 OK"));
    }

    #[test]
    #[ignore = "implement Response::to_bytes"]
    fn test_to_bytes_body_present() {
        let r = Response::ok("hello world");
        let bytes = r.to_bytes();
        let s = std::str::from_utf8(&bytes).unwrap();
        assert!(s.ends_with("hello world"));
    }

    #[test]
    #[ignore = "implement Response::header"]
    fn test_header_builder() {
        let r = Response::ok("").header("X-Custom", "value");
        assert_eq!(r.headers.get("x-custom"), Some(&"value".to_string()));
    }
}
