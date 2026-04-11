use http_server::{Router, Server};
use http_server::handler::handler;
use http_server::response::Response;

#[tokio::main]
async fn main() {
    println!("http-server — Capstone 3");
    println!("Implement the server to start accepting connections.");
    println!();
    println!("Modules to implement:");
    println!("  src/request.rs    — HTTP/1.1 request parser");
    println!("  src/response.rs   — Response builder");
    println!("  src/router.rs     — URL routing");
    println!("  src/middleware.rs — Logging, rate limiting");
    println!("  src/server.rs     — Async TCP listener");

    // Example of how the server will look when complete:
    // let router = Router::new()
    //     .get("/", handler(|_req| Ok(Response::ok("Hello, world!"))))
    //     .get("/health", handler(|_req| Ok(Response::ok("ok"))));
    // let server = Server::new("127.0.0.1:8080".parse().unwrap(), router);
    // server.run().await.unwrap();
}
