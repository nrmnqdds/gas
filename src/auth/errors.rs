//! Error types for the gomaluum-auth service
//!
//! This module defines all custom error types used throughout the authentication flow.
//! Uses thiserror for convenient error derivation and tonic::Status conversion.

use thiserror::Error;
use tonic::Status;

/// Custom error types for authentication operations
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Failed to create cookie jar")]
    CookieJarCreationFailed,

    #[error("Failed to parse URL: {0}")]
    URLParseFailed(#[from] url::ParseError),

    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to close request body")]
    FailedToCloseRequestBody,

    #[error("Failed to close response body")]
    FailedToCloseResponseBody,

    #[error("Login failed: Invalid credentials or authentication token not found")]
    LoginFailed,

    #[error("Authentication cookie not found")]
    AuthCookieNotFound,

    #[error("Invalid response from authentication server")]
    InvalidAuthResponse,

    #[error("Network timeout")]
    NetworkTimeout,

    #[error("Internal server error: {0}")]
    InternalError(String),
}

/// Convert AuthError to tonic::Status for gRPC responses
impl From<AuthError> for Status {
    fn from(error: AuthError) -> Self {
        match error {
            AuthError::LoginFailed | AuthError::AuthCookieNotFound => {
                Status::unauthenticated(error.to_string())
            }
            AuthError::URLParseFailed(_) | AuthError::InvalidAuthResponse => {
                Status::invalid_argument(error.to_string())
            }
            AuthError::NetworkTimeout => Status::deadline_exceeded(error.to_string()),
            AuthError::RequestFailed(_) => Status::unavailable(error.to_string()),
            _ => Status::internal(error.to_string()),
        }
    }
}

/// Result type alias for authentication operations
pub type AuthResult<T> = Result<T, AuthError>;
