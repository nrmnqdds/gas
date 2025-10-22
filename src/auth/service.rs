//! Authentication service module for i-Ma'luum login
//!
//! This module provides the authentication service implementation with optimized
//! HTTP request handling, cookie management, and error handling.

use log::{error, info, warn};
use reqwest::Client;
use std::collections::HashMap;
use url::Url;

use crate::{
    auth::{
        constants::{AUTH_COOKIE_NAME, IMALUUM_CAS_PAGE, IMALUUM_LOGIN_PAGE, IMALUUM_PAGE},
        errors::*,
    },
    http::client::{create_client_with_cookies, set_common_headers},
};

/// Authentication service for handling i-Ma'luum login operations
pub struct AuthService;

impl AuthService {
    /// Creates a new AuthService instance
    pub fn new() -> AuthResult<Self> {
        Ok(Self)
    }

    /// Performs login to i-Ma'luum and returns the authentication token
    ///
    /// This function makes two HTTP requests:
    /// 1. GET request to initialize session and get cookies
    /// 2. POST request with credentials to authenticate
    ///
    /// # Arguments
    /// * `username` - The user's username
    /// * `password` - The user's password
    ///
    /// # Returns
    /// * `Ok((token, username, password))` - Authentication successful, returns token and credentials
    /// * `Err(AuthError)` - Authentication failed or network error occurred
    ///
    /// # Performance Optimizations
    /// - Uses connection pooling via reusable client
    /// - Enables HTTP/2 and compression
    /// - Uses async/await for non-blocking I/O
    /// - Minimal allocations with string borrowing where possible
    pub async fn login(
        &self,
        username: String,
        password: String,
    ) -> AuthResult<(String, String, String)> {
        // Create client with cookie store for session management
        let client = create_client_with_cookies();

        // Prepare form data
        let form_data = self.create_form_data(&username, &password);

        // Execute the two-step authentication flow
        self.perform_authentication(&client, form_data).await?;

        // Extract authentication token from cookies
        let token = self.extract_auth_token(&client).await?;

        info!("Login successful for user: {}", username);
        Ok((token, username, password))
    }

    /// Creates form data for login request
    #[inline]
    fn create_form_data(&self, username: &str, password: &str) -> HashMap<&'static str, String> {
        let mut form = HashMap::with_capacity(5);
        form.insert("username", username.to_string());
        form.insert("password", password.to_string());
        form.insert("execution", "e1s1".to_string());
        form.insert("_eventId", "submit".to_string());
        form.insert("geolocation", String::new());
        form
    }

    /// Performs the two-step authentication flow
    ///
    /// Step 1: GET request to CAS page to initialize session
    /// Step 2: POST request with credentials to authenticate
    async fn perform_authentication(
        &self,
        client: &Client,
        form_data: HashMap<&str, String>,
    ) -> AuthResult<()> {
        // First request: GET to initialize session and obtain cookies
        info!("Step 1: Initializing session with CAS server");
        let first_request = set_common_headers(client.get(IMALUUM_CAS_PAGE));

        let first_response = first_request.send().await.map_err(|e| {
            error!("Failed to send first GET request to CAS: {:?}", e);
            error!(
                "Error details - kind: {:?}, url: {:?}",
                e.to_string(),
                e.url()
            );
            AuthError::RequestFailed(e)
        })?;

        let first_status = first_response.status();
        let first_url = first_response.url().clone();

        info!(
            "First request completed - Status: {}, Final URL: {}",
            first_status, first_url
        );

        if !first_status.is_success() && !first_status.is_redirection() {
            warn!("First request returned unexpected status: {}", first_status);
        }

        // Cookies are automatically stored in the client's cookie store
        // We must consume the response body to ensure cookies are properly saved
        let _ = first_response.text().await.map_err(|e| {
            error!("Failed to read first response body: {}", e);
            AuthError::RequestFailed(e)
        })?;

        info!("Step 2: Submitting credentials");

        // Second request: POST with credentials
        // Add Referer header to mimic browser behavior
        let second_request = set_common_headers(client.post(IMALUUM_LOGIN_PAGE))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Referer", IMALUUM_CAS_PAGE)
            .header("Origin", "https://cas.iium.edu.my:8448")
            .form(&form_data);

        let second_response = second_request.send().await.map_err(|e| {
            error!(
                "Failed to send second POST request with credentials: {:?}",
                e
            );
            error!(
                "Error details - kind: {:?}, url: {:?}",
                e.to_string(),
                e.url()
            );
            AuthError::RequestFailed(e)
        })?;

        let second_status = second_response.status();
        let second_url = second_response.url().clone();

        info!(
            "Second request completed - Status: {}, Final URL: {}",
            second_status, second_url
        );

        // Read the response body to ensure cookies are set
        let response_body = second_response.text().await.map_err(|e| {
            error!("Failed to read second response body: {}", e);
            AuthError::RequestFailed(e)
        })?;

        // Check if login was successful by looking for error indicators in the response
        if response_body.contains("Login failed") || response_body.contains("Invalid credentials") {
            error!("Login failed: Invalid credentials detected in response");
            return Err(AuthError::LoginFailed);
        }

        if !second_status.is_success() && !second_status.is_redirection() {
            error!("Second request returned error status: {}", second_status);
            return Err(AuthError::LoginFailed);
        }

        info!("Authentication flow completed successfully");
        Ok(())
    }

    /// Extracts the MOD_AUTH_CAS authentication token from cookies
    async fn extract_auth_token(&self, client: &Client) -> AuthResult<String> {
        // Make a request to get cookies from the client's cookie store
        // The cookie store in reqwest automatically includes cookies in requests
        let url = Url::parse(IMALUUM_PAGE).map_err(|e| {
            error!("Failed to parse base URL: {}", e);
            AuthError::URLParseFailed(e)
        })?;

        let response = client.get(url).send().await.map_err(|e| {
            error!("Failed to get cookies from base URL: {}", e);
            AuthError::RequestFailed(e)
        })?;

        // Check cookies in the response - this is the most reliable way
        for cookie in response.cookies() {
            if cookie.name() == AUTH_COOKIE_NAME {
                return Ok(cookie.value().to_string());
            }
        }

        error!("Authentication cookie '{}' not found", AUTH_COOKIE_NAME);
        Err(AuthError::AuthCookieNotFound)
    }
}

impl Default for AuthService {
    fn default() -> Self {
        Self::new().expect("Failed to create AuthService with default settings")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_service_creation() {
        let service = AuthService::new();
        assert!(service.is_ok());
    }

    #[test]
    fn test_form_data_creation() {
        let service = AuthService::new().unwrap();
        let form = service.create_form_data("testuser", "testpass");

        assert_eq!(form.get("username").unwrap(), "testuser");
        assert_eq!(form.get("password").unwrap(), "testpass");
        assert_eq!(form.get("execution").unwrap(), "e1s1");
        assert_eq!(form.get("_eventId").unwrap(), "submit");
        assert_eq!(form.get("geolocation").unwrap(), "");
    }

    #[tokio::test]
    async fn test_login_with_invalid_credentials() {
        let service = AuthService::new().unwrap();
        let result = service
            .login("invalid_user".to_string(), "invalid_pass".to_string())
            .await;

        // This should fail with invalid credentials
        // Note: This is a live test and may not work in CI/CD
        // In production, you'd mock the HTTP client
        assert!(result.is_err());
    }
}
