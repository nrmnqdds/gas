//! GoMaluum Authentication Service
//!
//! A high-performance gRPC authentication service for i-Ma'luum login operations.
//! This service provides optimized HTTP client handling with connection pooling,
//! cookie management, and efficient async I/O.

pub mod auth;
pub mod http;
pub mod middleware;

use crate::auth::grpc::GRPCServer;
use crate::auth::grpc::auth_proto::auth_server::AuthServer;
use crate::middleware::pb::echo_server::EchoServer as EchoService;
use crate::middleware::{EchoServer, check_auth};
use console::Style;
use dotenvy::dotenv;
use log::{error, info};
use std::env;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logger
    env_logger::init();

    // Get bind address from environment or use default
    let addr = env::var("BIND_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:50052".to_string())
        .parse()?;

    // Create gRPC servers
    let auth_server = GRPCServer::new().map_err(|e| {
        error!("Failed to create auth server: {}", e);
        e
    })?;

    let echo_server = EchoServer::default();

    info!("Initializing gRPC services...");

    // Build the gRPC server with both services
    let auth_service = AuthServer::new(auth_server);
    let echo_service = EchoService::with_interceptor(echo_server, check_auth);

    print_intro();

    // Start the server
    Server::builder()
        .add_service(auth_service)
        .add_service(echo_service)
        .serve(addr)
        .await?;

    Ok(())
}

fn print_intro() {
    println!(
        "{}",
        Style::new().red().apply_to(
            r"
 ██████╗  █████╗ ███████╗
██╔════╝ ██╔══██╗██╔════╝
██║  ███╗███████║███████╗
██║   ██║██╔══██║╚════██║
╚██████╔╝██║  ██║███████║
 ╚═════╝ ╚═╝  ╚═╝╚══════╝
GoMaluum Authentication Service
"
        )
    );
    println!(
        "{}",
        Style::new().yellow().apply_to(
            "===================================================================================="
        )
    );
    println!(
        "{}",
        Style::new()
            .blue()
            .apply_to("gRPC server listening on [::1]:50052")
    );
    println!(
        "{}",
        Style::new().yellow().apply_to(
            "===================================================================================="
        )
    );
}
