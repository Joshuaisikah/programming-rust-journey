// server.rs — Async TCP listener and connection dispatcher
//
// Binds to a TCP port, accepts connections in async tasks,
// reads one HTTP request per connection, dispatches to the Router,
// and writes the response back.

use std::net::SocketAddr;
use tokio::net::TcpListener;
use crate::error::HttpError;
use crate::router::Router;

pub struct Server {
    addr: SocketAddr,
    router: Router,
}

impl Server {
    /// Create a server bound to the given address with the given router.
    pub fn new(addr: impl Into<SocketAddr>, router: Router) -> Self {
        todo!("Server {{ addr: addr.into(), router }}")
    }

    /// Start accepting connections. Runs until the process is killed.
    /// Each connection is handled in a separate tokio task.
    pub async fn run(self) -> Result<(), HttpError> {
        todo!("TcpListener::bind, loop accept, spawn task per connection")
    }

    /// The address this server will listen on.
    pub fn addr(&self) -> SocketAddr {
        todo!("self.addr")
    }
}

/// Handle a single TCP connection: read request, dispatch, write response.
async fn handle_connection(
    mut stream: tokio::net::TcpStream,
    router: std::sync::Arc<Router>,
) -> Result<(), HttpError> {
    todo!("read bytes from stream, parse Request, dispatch, write Response::to_bytes()")
}

#[cfg(test)]
mod tests {
    // Integration tests for the server live in tests/integration_tests.rs
    // Unit tests here cover helpers only.

    use super::*;

    #[test]
    #[ignore = "implement Server::addr"]
    fn test_server_stores_addr() {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let s = Server::new(addr, Router::new());
        assert_eq!(s.addr().port(), 8080);
    }
}
