//! Example gRPC client for the GoMaluum Authentication Service
//!
//! This example demonstrates how to connect to the authentication service
//! and perform a login operation.
//!
//! Usage:
//! ```bash
//! cargo run --example client -- <username> <password>
//! ```

use std::env;

// Include the generated protobuf code
pub mod auth_proto {
    tonic::include_proto!("grpc.gas.auth");
}

use auth_proto::auth_client::AuthClient;
use auth_proto::
