// capstone-03 — Async HTTP/1.1 Server
//
// ── HOW ONE REQUEST FLOWS THROUGH THE SERVER ─────────────────
//
//   Server::run()
//       ↓  TcpListener::bind, then loop { listener.accept() }
//       ↓  each connection is spawned as its own tokio task
//   handle_connection(stream, Arc<Router>)
//       ↓  read raw bytes from TcpStream
//   Request::parse(bytes)
//       ↓  "GET /users/42 HTTP/1.1\r\nHost: ...\r\n\r\n"
//       ↓  → Request { method: Get, path: "/users/42", headers, body }
//   Router::dispatch(&mut req)
//       ↓  split path into segments: ["users", "42"]
//       ↓  match against registered patterns: ["users", ":id"]
//       ↓  extract params: { "id" → "42" }, write into req.params
//       ↓  call the matching Handler closure
//   Handler (your closure)
//       ↓  req.params.get("id") → "42"
//       ↓  returns Ok(Response::json("..."))
//   Middleware (Logger, RateLimiter)
//       ↓  wraps the handler call, logs it or rejects it
//   Response::to_bytes()
//       ↓  "HTTP/1.1 200 OK\r\nContent-Type: ...\r\n\r\nbody"
//   write bytes back to TcpStream
//
// ── IMPLEMENTATION ORDER ──────────────────────────────────────
//
//   1. error.rs     — HttpError variants: BadRequest, NotFound, MethodNotAllowed, etc.
//                     Every module returns Result<_, HttpError>. Start here.
//
//   2. response.rs  — StatusCode::reason_phrase, Response::new/ok/json/not_found
//                     Response::header (builder), Response::to_bytes
//                     Write tests: does to_bytes() start with "HTTP/1.1 200 OK"?
//
//   3. request.rs   — Method::parse, Method::as_str
//                     Request::parse: split on "\r\n\r\n", parse request line,
//                     parse headers (split each line on ": "), read body up to
//                     Content-Length bytes. Query string: split path on "?".
//
//   4. handler.rs   — FnHandler::new, Handler impl for FnHandler
//                     handler() convenience wrapper
//                     (likely already compilable — check it compiles)
//
//   5. router.rs    — split_path, match_path (literal vs :param)
//                     Router::new, get/post/put/delete (register route)
//                     Router::dispatch: find matching route, fill req.params, call handler
//
//   6. middleware.rs — Logger::process: call next, then log method + path + status
//                      RateLimiter::process: count requests, return 429 if over limit
//
//   7. server.rs    — Server::new, Server::run: bind, accept loop, spawn tasks
//                     handle_connection: read → parse → dispatch → write
//                     Wrap Router in Arc so it can be shared across tasks.
//
// ── ONCE COMPLETE ─────────────────────────────────────────────

use http_server::{Router, Server};
use http_server::handler::handler;
use http_server::middleware::Logger;
use http_server::response::Response;

#[tokio::main]
async fn main() {
    println!("http-server — Capstone 3");
    println!();
    println!("Run `cargo test -p capstone-03-http-server` to track progress.");
    println!("Once server.rs is done, uncomment below and visit http://127.0.0.1:8080");

    // Uncomment once implemented:
    //
    // let router = Router::new()
    //     .get("/", handler(|_req| {
    //         Ok(Response::ok("Welcome to rserver!"))
    //     }))
    //     .get("/health", handler(|_req| {
    //         Ok(Response::json(r#"{"status":"ok"}"#))
    //     }))
    //     .get("/users/:id", handler(|req| {
    //         let id = req.params.get("id").cloned().unwrap_or_default();
    //         Ok(Response::json(format!(r#"{{"id":"{}"}}"#, id)))
    //     }))
    //     .post("/users", handler(|req| {
    //         let body = req.body_str().unwrap_or("{}");
    //         println!("Creating user: {}", body);
    //         Ok(Response::ok("created"))
    //     }));
    //
    // println!("Listening on http://127.0.0.1:8080");
    // let server = Server::new("127.0.0.1:8080".parse().unwrap(), router);
    // server.run().await.unwrap();
}
