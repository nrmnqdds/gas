//! gRPC service implementation for authentication
//!
//! This module provides the gRPC server implementation that integrates with
//! the AuthService to handle login requests via gRPC protocol.

use log::{error, info};
use tonic::{Request, Response, Status};

// Import generated protobuf code
pub mod auth_proto {
    tonic::include_proto!("grpc.gas.auth");
}

use auth_proto::auth_server::Auth;
use auth_proto::{LoginRequest, LoginResponse};

use crate::auth::errors::AuthError;
use crate::auth::service::AuthService;

/// gRPC server implementation for authentication service
pub struct GRPCServer {
    auth_service: AuthService,
}

impl GRPCServer {
    /// Creates a new GRPCServer instance
    pub fn new() -> Result<Self, AuthError> {
        let auth_service = AuthService::new()?;
        Ok(Self { auth_service })
    }
}

impl Default for GRPCServer {
    fn default() -> Self {
        Self::new().expect("Failed to create GRPCServer with default settings")
    }
}

#[tonic::async_trait]
impl Auth for GRPCServer {
    /// Handles login requests via gRPC
    ///
    /// This method receives login credentials via gRPC, performs authentication
    /// through the AuthService, and returns the authentication token.
    ///
    /// # Arguments
    /// * `request` - gRPC request containing LoginRequest with username and password
    ///
    /// # Returns
    /// * `Ok(Response<LoginResponse>)` - Successful authentication with token
    /// * `Err(Status)` - Authentication failed or error occurred
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();

        info!("Login request received for user: {}", req.username);

        // Validate input
        if req.username.is_empty() {
            error!("Login failed: Empty username");
            return Err(Status::invalid_argument("Username cannot be empty"));
        }

        if req.password.is_empty() {
            error!("Login failed: Empty password");
            return Err(Status::invalid_argument("Password cannot be empty"));
        }

        // Perform authentication
        match self
            .auth_service
            .login(req.username.clone(), req.password.clone())
            .await
        {
            Ok((token, username, password)) => {
                info!("Login successful for user: {}", username);

                let response = LoginResponse {
                    token,
                    username,
                    password,
                };

                Ok(Response::new(response))
            }
            Err(e) => {
                error!("Login failed for user {}: {:?}", req.username, e);
                Err(Status::from(e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grpc_server_creation() {
        let server = GRPCServer::new();
        assert!(server.is_ok());
    }

    #[tokio::test]
    async fn test_login_empty_username() {
        let server = GRPCServer::new().unwrap();
        let request = Request::new(LoginRequest {
            username: String::new(),
            password: "password".to_string(),
        });

        let result = server.login(request).await;
        assert!(result.is_err());

        if let Err(status) = result {
            assert_eq!(status.code(), tonic::Code::InvalidArgument);
        }
    }

    #[tokio::test]
    async fn test_login_empty_password() {
        let server = GRPCServer::new().unwrap();
        let request = Request::new(LoginRequest {
            username: "username".to_string(),
            password: String::new(),
        });

        let result = server.login(request).await;
        assert!(result.is_err());

        if let Err(status) = result {
            assert_eq!(status.code(), tonic::Code::InvalidArgument);
        }
    }
}
