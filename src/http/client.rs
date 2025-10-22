//! Reusable HTTPS client module optimized for performance
//!
//! This module provides a singleton HTTP client with connection pooling,
//! cookie management, and optimized settings for high-performance requests.

use once_cell::sync::Lazy;
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

/// Global shared HTTP client instance with optimized settings
///
/// Uses connection pooling and compression for optimal performance.
/// The client is thread-safe and can be shared across the application.
pub static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    ClientBuilder::new()
        // Connection pooling settings
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(Duration::from_secs(90))
        // Timeout settings - i-Ma'luum can be slow
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(30))
        // Enable compression
        .gzip(true)
        .brotli(true)
        .deflate(true)
        // TCP settings for better performance
        .tcp_nodelay(true)
        .tcp_keepalive(Duration::from_secs(60))
        // Redirect policy - follow redirects automatically
        .redirect(reqwest::redirect::Policy::limited(10))
        // Disable HTTP/2 prior knowledge - let negotiation happen naturally
        .http1_only()
        .build()
        .expect("Failed to build HTTP client")
});

/// Creates a new HTTP client with cookie jar support
///
/// This client maintains cookies across requests, useful for authenticated sessions.
/// It uses the same optimized settings as the global client.
pub fn create_client_with_cookies() -> Client {
    ClientBuilder::new()
        // Enable cookie store
        .cookie_store(true)
        // Connection pooling settings
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(Duration::from_secs(90))
        // Timeout settings - i-Ma'luum can be slow
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(30))
        // Enable compression
        .gzip(true)
        .brotli(true)
        .deflate(true)
        // TCP settings for better performance
        .tcp_nodelay(true)
        .tcp_keepalive(Duration::from_secs(60))
        // Redirect policy - follow redirects automatically
        .redirect(reqwest::redirect::Policy::none())
        // Disable HTTP/2 prior knowledge - let negotiation happen naturally
        .http1_only()
        // Danger: Accept invalid certificates (i-Ma'luum may have cert issues)
        // Remove this in production if certificates are valid
        .danger_accept_invalid_certs(false)
        .default_headers(set_common_headers())
        .build()
        .expect("Failed to build HTTP client with cookies")
}

/// Sets common headers for i-Ma'luum requests
///
/// These headers mimic a real browser to avoid being blocked by the server
pub fn set_common_headers() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
    headers.insert("Accept-Language", "en-US,en;q=0.9".parse().unwrap());
    headers.insert("Accept-Encoding", "gzip, deflate, br".parse().unwrap());
    headers.insert("Cache-Control", "no-cache".parse().unwrap());
    headers.insert("Pragma", "no-cache".parse().unwrap());
    headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());
    headers.insert("Sec-Fetch-Dest", "document".parse().unwrap());
    headers.insert("Sec-Fetch-Mode", "navigate".parse().unwrap());
    headers.insert("Sec-Fetch-Site", "none".parse().unwrap());
    headers.insert("Sec-Fetch-User", "?1".parse().unwrap());
    headers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_client_creation() {
        let client = &*HTTP_CLIENT;
        assert!(client.get("https://example.com").build().is_ok());
    }

    #[test]
    fn test_client_with_cookies_creation() {
        let client = create_client_with_cookies();
        assert!(client.get("https://example.com").build().is_ok());
    }

    #[tokio::test]
    async fn test_http_client_request() {
        let client = &*HTTP_CLIENT;
        let result = client.get("https://httpbin.org/get").send().await;
        assert!(result.is_ok());
    }
}
