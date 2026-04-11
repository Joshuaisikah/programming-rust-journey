// middleware.rs — Middleware trait and built-in middlewares
//
// Middleware wraps a handler to add cross-cutting behaviour:
//   - Logging: print request method/path/status/duration
//   - Rate limiting: reject requests over a threshold per IP

use crate::error::HttpError;
use crate::handler::Handler;
use crate::request::Request;
use crate::response::Response;

/// A middleware wraps an inner Handler and can inspect/modify
/// the request before and/or the response after.
pub trait Middleware: Send + Sync + 'static {
    fn process(
        &self,
        req: &mut Request,
        next: &dyn Handler,
    ) -> Result<Response, HttpError>;
}

// ── Logger ────────────────────────────────────────────────────

/// Logs "METHOD path → STATUS" for every request to stderr.
pub struct Logger;

impl Middleware for Logger {
    fn process(&self, req: &mut Request, next: &dyn Handler) -> Result<Response, HttpError> {
        todo!("call next.handle(req), print method + path + status, return response")
    }
}

// ── RateLimiter ───────────────────────────────────────────────

/// Rejects requests when more than `max_per_second` arrive per second.
/// Uses a token-bucket approximation stored in memory.
pub struct RateLimiter {
    max_per_window: usize,
}

impl RateLimiter {
    pub fn new(max_per_window: usize) -> Self {
        RateLimiter { max_per_window }
    }
}

impl Middleware for RateLimiter {
    fn process(&self, req: &mut Request, next: &dyn Handler) -> Result<Response, HttpError> {
        todo!("check request count, return 429 if over limit, else call next")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handler::handler;
    use crate::request::Method;
    use crate::response::StatusCode;

    fn fake_req() -> Request {
        Request {
            method: Method::Get,
            path: "/test".to_string(),
            query: Default::default(),
            headers: Default::default(),
            body: vec![],
            params: Default::default(),
        }
    }

    #[test]
    #[ignore = "implement Logger::process"]
    fn test_logger_passes_through_response() {
        let inner = handler(|_| Ok(Response::ok("OK")));
        let logger = Logger;
        let mut req = fake_req();
        let resp = logger.process(&mut req, inner.as_ref()).unwrap();
        assert_eq!(resp.status, StatusCode::Ok);
    }

    #[test]
    #[ignore = "implement Logger::process"]
    fn test_logger_propagates_errors() {
        let inner = handler(|_| Err(HttpError::MethodNotAllowed));
        let logger = Logger;
        let mut req = fake_req();
        assert!(logger.process(&mut req, inner.as_ref()).is_err());
    }
}
