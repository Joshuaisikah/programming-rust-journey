// router.rs — URL routing with path parameters
//
// Routes look like: GET /users/:id/posts/:post_id
// The router matches an incoming path and extracts named parameters
// into req.params before calling the handler.

use std::collections::HashMap;
use crate::error::HttpError;
use crate::handler::Handler;
use crate::request::{Method, Request};
use crate::response::Response;

/// One registered route: method + pattern + handler.
struct Route {
    method: Method,
    /// Pattern segments, e.g. ["users", ":id", "posts"]
    segments: Vec<String>,
    handler: Box<dyn Handler>,
}

pub struct Router {
    routes: Vec<Route>,
    not_found_handler: Option<Box<dyn Handler>>,
}

impl Router {
    pub fn new() -> Self {
        todo!("Router {{ routes: vec![], not_found_handler: None }}")
    }

    /// Register a GET handler for the given path pattern.
    pub fn get(self, pattern: &str, handler: Box<dyn Handler>) -> Self {
        todo!("parse pattern into segments, push Route, return self")
    }

    /// Register a POST handler.
    pub fn post(self, pattern: &str, handler: Box<dyn Handler>) -> Self {
        todo!("like get but with Method::Post")
    }

    /// Register a PUT handler.
    pub fn put(self, pattern: &str, handler: Box<dyn Handler>) -> Self {
        todo!("like get but with Method::Put")
    }

    /// Register a DELETE handler.
    pub fn delete(self, pattern: &str, handler: Box<dyn Handler>) -> Self {
        todo!("like get but with Method::Delete")
    }

    /// Set a custom 404 handler.
    pub fn not_found(mut self, handler: Box<dyn Handler>) -> Self {
        todo!("self.not_found_handler = Some(handler); self")
    }

    /// Dispatch an incoming request. Fills req.params on match.
    pub fn dispatch(&self, req: &mut Request) -> Result<Response, HttpError> {
        todo!("find matching route, extract params, call handler.handle(req)")
    }
}

/// Try to match `path_segments` against a `pattern_segments`.
/// Returns Some(params) if they match, None otherwise.
fn match_path(
    pattern: &[String],
    path_segments: &[&str],
) -> Option<HashMap<String, String>> {
    todo!("walk both slices: literal must equal, :param captures to map")
}

/// Split a URL path into segments, ignoring leading/trailing slashes.
fn split_path(path: &str) -> Vec<&str> {
    todo!("path.split('/').filter(|s| !s.is_empty()).collect()")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handler::handler;
    use crate::response::StatusCode;

    fn get_req(path: &str) -> Request {
        Request {
            method: Method::Get,
            path: path.to_string(),
            query: Default::default(),
            headers: Default::default(),
            body: vec![],
            params: Default::default(),
        }
    }

    // ── split_path ────────────────────────────────────────────

    #[test]
    #[ignore = "implement split_path"]
    fn test_split_path_basic() {
        assert_eq!(split_path("/users/42/posts"), vec!["users", "42", "posts"]);
    }

    #[test]
    #[ignore = "implement split_path"]
    fn test_split_path_root() {
        assert_eq!(split_path("/"), Vec::<&str>::new());
    }

    // ── match_path ────────────────────────────────────────────

    #[test]
    #[ignore = "implement match_path"]
    fn test_match_literal_path() {
        let pattern = vec!["users".to_string(), "list".to_string()];
        let result = match_path(&pattern, &["users", "list"]);
        assert!(result.is_some());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    #[ignore = "implement match_path"]
    fn test_match_with_param() {
        let pattern = vec!["users".to_string(), ":id".to_string()];
        let result = match_path(&pattern, &["users", "42"]).unwrap();
        assert_eq!(result.get("id"), Some(&"42".to_string()));
    }

    #[test]
    #[ignore = "implement match_path"]
    fn test_no_match_different_length() {
        let pattern = vec!["users".to_string()];
        assert!(match_path(&pattern, &["users", "extra"]).is_none());
    }

    #[test]
    #[ignore = "implement match_path"]
    fn test_no_match_literal_mismatch() {
        let pattern = vec!["users".to_string()];
        assert!(match_path(&pattern, &["posts"]).is_none());
    }

    // ── Router::dispatch ──────────────────────────────────────

    #[test]
    #[ignore = "implement Router::dispatch"]
    fn test_dispatch_simple_route() {
        let router = Router::new()
            .get("/hello", handler(|_| Ok(Response::ok("world"))));
        let mut req = get_req("/hello");
        let resp = router.dispatch(&mut req).unwrap();
        assert_eq!(resp.status, StatusCode::Ok);
    }

    #[test]
    #[ignore = "implement Router::dispatch"]
    fn test_dispatch_extracts_params() {
        let router = Router::new()
            .get("/users/:id", handler(|req| {
                let id = req.params.get("id").cloned().unwrap_or_default();
                Ok(Response::ok(id))
            }));
        let mut req = get_req("/users/42");
        router.dispatch(&mut req).unwrap();
        assert_eq!(req.params.get("id"), Some(&"42".to_string()));
    }

    #[test]
    #[ignore = "implement Router::dispatch"]
    fn test_dispatch_not_found() {
        let router = Router::new();
        let mut req = get_req("/missing");
        let result = router.dispatch(&mut req);
        assert!(result.is_err());
    }
}
