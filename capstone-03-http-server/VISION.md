# Capstone 03 — Async HTTP/1.1 Server

## What You're Building

A production-style HTTP server built from scratch on top of raw TCP sockets.
No web framework — you build the framework itself.

```
GET /users/42 HTTP/1.1
Host: localhost:8080

→ 200 OK
   {"id": 42, "name": "Joshua"}
```

Features: request parsing, path routing with parameters (`/users/:id`),
a middleware chain for logging and rate-limiting, static file serving,
and graceful shutdown.

## The Real Point

Every web framework you will ever use (Axum, Actix, Rocket) does exactly
what you are building here under the hood. This capstone removes the magic:

- **HTTP is just text over TCP** — an HTTP request is bytes arriving over a socket.
  You write the parser that turns those bytes into a structured `Request` type.

- **Async and Tokio** — your server needs to handle thousands of connections at once
  without blocking. Tokio's async runtime lets each connection yield while waiting
  for I/O, so one thread serves many clients.

- **Trait objects for middleware** — each middleware implements a `Handler` trait.
  The router chains them at runtime without knowing their concrete types.
  This is Rust's version of polymorphism.

- **Graceful shutdown** — when you press Ctrl-C, the server finishes in-flight
  requests before exiting.

## Architecture: How the Files Connect

```
main.rs
  │
  │  builds a Router, registers handlers, starts the Server
  │  wires the shutdown signal
  │
  └── server.rs      ← the TCP listener loop
        │                binds to a port, accepts connections
        │                spawns a tokio task per connection
        │                passes each connection to the Router
        │
        ├── router.rs    ← maps (method, path) → handler
        │     │              supports path parameters: /users/:id
        │     │              chains middleware around each handler
        │     │
        │     ├── handler.rs     ← the Handler trait
        │     │                     every route handler and middleware implements this
        │     │                     one method: async fn handle(req) → Response
        │     │
        │     └── middleware.rs  ← built-in middleware: logger, rate limiter
        │                           each wraps a Handler and calls it after doing its work
        │
        ├── request.rs   ← parses raw HTTP/1.1 bytes into a typed Request
        │                   method (GET/POST/...), path, headers, body
        │                   the only place that touches raw bytes
        │
        ├── response.rs  ← builds HTTP/1.1 response bytes from a typed Response
        │                   status code, headers, body
        │                   the only place that serializes back to bytes
        │
        └── error.rs     ← HttpError: parse errors, not found, method not allowed
                            used by request.rs, router.rs, handler.rs
```

The key insight: `request.rs` and `response.rs` are the **two boundaries** between
raw bytes and Rust types. Everything between them works with typed values, never
raw strings or byte slices.

## Why Each File Exists

**`main.rs`** — Configuration only. Registers routes, starts the server, handles
the shutdown signal. Contains no HTTP logic so it reads like a config file.

**`server.rs`** — The TCP loop: `TcpListener::bind`, `accept()`, spawn a task.
Separated from routing because accepting connections is a different concern than
deciding what to do with them. Also handles graceful shutdown by tracking
in-flight tasks.

**`request.rs`** — HTTP/1.1 request parsing is its own well-defined problem.
Separating it means you can test parsing against raw byte strings without needing
a running server. It also means changing the HTTP version (HTTP/2) only touches
this file.

**`response.rs`** — The mirror of `request.rs`. Builds the bytes that go back
to the client. Separated so handler logic only constructs typed `Response` values
and never manually formats HTTP headers.

**`router.rs`** — Matches incoming requests to the right handler. Handles path
parameter extraction (`/users/42` → `id = 42`) and method matching.
Separated from `server.rs` because the routing table can be tested entirely in
memory without accepting a real TCP connection.

**`handler.rs`** — Defines the `Handler` trait that every route handler and
middleware implements. Separated into its own file so it can be imported by
`router.rs`, `middleware.rs`, and any user-defined handler without creating
circular dependencies.

**`middleware.rs`** — Each middleware wraps a `Handler` and adds behavior before
or after calling it. Separated from `handler.rs` so middleware implementations
can grow independently and be composed freely.

**`error.rs`** — HTTP-level errors in one place: malformed requests, unknown routes,
wrong methods. Separated so any module can return `HttpError` without importing
unrelated types.

## Chapters It Combines

Ch11 Traits & Generics · Ch18 I/O · Ch19 Concurrency · Ch20 Async

## Mental Model

Think of the core of Axum or Nginx. You are building the layer between raw TCP
and application code. After this capstone, when someone says "we use Axum for
our API server," you will know exactly what Axum is doing on your behalf.
