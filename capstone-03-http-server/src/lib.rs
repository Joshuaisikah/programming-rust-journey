// Capstone 3 — Async HTTP/1.1 Server
//
// A production-style async HTTP server built on tokio:
//   - HTTP/1.1 request parsing over TCP
//   - Router with path parameters (/users/:id)
//   - Middleware chain (logging, rate-limiting)
//   - Static file serving
//   - Graceful shutdown

pub mod error;
pub mod handler;
pub mod middleware;
pub mod request;
pub mod response;
pub mod router;
pub mod server;

pub use error::HttpError;
pub use handler::Handler;
pub use request::{Method, Request};
pub use response::{Response, StatusCode};
pub use router::Router;
pub use server::Server;
