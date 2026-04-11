// handler.rs — Handler trait
//
// A Handler is anything that can process a Request and produce a Response.
// Closures automatically implement Handler via a blanket impl.

use crate::error::HttpError;
use crate::request::Request;
use crate::response::Response;

/// A synchronous request handler.
pub trait Handler: Send + Sync + 'static {
    fn handle(&self, req: &Request) -> Result<Response, HttpError>;
}

/// Wrap a closure as a Handler.
pub struct FnHandler<F> {
    f: F,
}

impl<F> FnHandler<F>
where
    F: Fn(&Request) -> Result<Response, HttpError> + Send + Sync + 'static,
{
    pub fn new(f: F) -> Self {
        FnHandler { f }
    }
}

impl<F> Handler for FnHandler<F>
where
    F: Fn(&Request) -> Result<Response, HttpError> + Send + Sync + 'static,
{
    fn handle(&self, req: &Request) -> Result<Response, HttpError> {
        (self.f)(req)
    }
}

/// Convenience function: wrap a closure into a boxed Handler.
pub fn handler<F>(f: F) -> Box<dyn Handler>
where
    F: Fn(&Request) -> Result<Response, HttpError> + Send + Sync + 'static,
{
    Box::new(FnHandler::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::StatusCode;

    fn fake_request() -> Request {
        Request {
            method: crate::request::Method::Get,
            path: "/".to_string(),
            query: Default::default(),
            headers: Default::default(),
            body: vec![],
            params: Default::default(),
        }
    }

    #[test]
    #[ignore = "implement Response::ok"]
    fn test_fn_handler_wraps_closure() {
        let h = handler(|_req| Ok(Response::ok("hello")));
        let req = fake_request();
        let resp = h.handle(&req).unwrap();
        assert_eq!(resp.status, StatusCode::Ok);
    }

    #[test]
    fn test_handler_can_return_error() {
        let h = handler(|_req| Err(HttpError::MethodNotAllowed));
        let req = fake_request();
        assert!(h.handle(&req).is_err());
    }
}
