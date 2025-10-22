# GoMaluum Authentication Service

A high-performance gRPC authentication service for i-Ma'luum login operations, written in Rust. This service provides optimized HTTP client handling with connection pooling, cookie management, and efficient async I/O.

## Features

- 🚀 **High Performance**: Optimized with connection pooling, HTTP/2, and async I/O
- 🔒 **Secure**: Handles authentication cookies securely
- 🔄 **Reusable HTTP Client**: Singleton pattern with shared connection pool
- 📦 **gRPC API**: Modern protocol buffers interface
- 🛡️ **Robust Error Handling**: Comprehensive error types with detailed logging
- ⚡ **Efficient**: Minimal allocations and optimized for throughput

## Architecture

### Performance Optimizations

1. **Connection Pooling**: Maintains up to 10 idle connections per host
2. **HTTP/2**: Enables multiplexing for better performance
3. **Compression**: Supports gzip, brotli, and deflate
4. **TCP Settings**:
   - `TCP_NODELAY` for reduced latency
   - TCP keepalive for long-lived connections
5. **Async I/O**: Non-blocking operations with Tokio runtime
6. **Zero-Copy Operations**: Minimal string allocations

## Installation

### Prerequisites

- Rust 1.70+ (with 2024 edition support)
- Protocol Buffers compiler (protoc)

### Build

```bash
cargo build --release
```

### Run

```bash
# With default settings (binds to [::1]:50052)
cargo run --release

# With custom bind address
BIND_ADDR="0.0.0.0:50051" cargo run --release

# With logging
RUST_LOG=info cargo run --release
```

## Usage

### gRPC API

The service exposes an `Auth` service with a `Login` method.

#### Protocol Buffers Definition

```protobuf
syntax = "proto3";

package grpc.gas.auth;

service Auth {
  rpc Login(LoginRequest) returns (LoginResponse) {};
}

message LoginRequest {
  string username = 1;
  string password = 2;
}

message LoginResponse {
  string token = 1;
  string username = 2;
  string password = 3;
}
```

## Authentication Flow

The login process follows a two-step authentication flow:

1. **Initialize Session**: GET request to CAS page to establish session cookies
2. **Authenticate**: POST request with credentials to obtain MOD_AUTH_CAS token

```
┌─────────┐                 ┌──────────────┐                 ┌─────────────┐
│ Client  │                 │  Auth Service │                 │  i-Ma'luum  │
└────┬────┘                 └───────┬───────┘                 └──────┬──────┘
     │                              │                                 │
     │  gRPC Login(user, pass)      │                                 │
     ├─────────────────────────────>│                                 │
     │                              │  GET /cas/login                 │
     │                              ├────────────────────────────────>│
     │                              │  Set-Cookie: SESSION            │
     │                              │<────────────────────────────────┤
     │                              │                                 │
     │                              │  POST /cas/login (credentials)  │
     │                              ├────────────────────────────────>│
     │                              │  Set-Cookie: MOD_AUTH_CAS       │
     │                              │<────────────────────────────────┤
     │                              │                                 │
     │  LoginResponse(token)        │                                 │
     │<─────────────────────────────┤                                 │
     │                              │                                 │
```

## Configuration

### Environment Variables

- `BIND_ADDR`: Server bind address (default: `[::1]:50052`)
- `RUST_LOG`: Logging level (e.g., `debug`, `info`, `warn`, `error`)

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_auth_service_creation

# Run with logging
RUST_LOG=debug cargo test -- --nocapture
```

## Troubleshooting

### Build Errors

If you encounter build errors:

```bash
# Update dependencies
cargo update

# Clean and rebuild
cargo clean && cargo build

# Check for missing tools
which protoc
```

### Connection Errors

If login fails:

1. Check network connectivity to i-Ma'luum servers
2. Verify credentials are correct
3. Check if i-Ma'luum servers are accessible
4. Enable debug logging: `RUST_LOG=debug cargo run`
