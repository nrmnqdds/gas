//! Constants module for i-Ma'luum authentication URLs and configuration

/// i-Ma'luum main page URL
pub const IMALUUM_PAGE: &str = "https://imaluum.iium.edu.my/";

/// i-Ma'luum CAS (Central Authentication Service) page URL
pub const IMALUUM_CAS_PAGE: &str =
    "https://cas.iium.edu.my:8448/cas/login?service=https%3a%2f%2fimaluum.iium.edu.my%2fhome";

/// i-Ma'luum login page URL for form submission
pub const IMALUUM_LOGIN_PAGE: &str = "https://cas.iium.edu.my:8448/cas/login?service=https%3a%2f%2fimaluum.iium.edu.my%2fhome?service=https%3a%2f%2fimaluum.iium.edu.my%2fhome";

/// Cookie name for MOD_AUTH_CAS authentication token
pub const AUTH_COOKIE_NAME: &str = "MOD_AUTH_CAS";

/// Default timeout for HTTP requests (in seconds)
pub const REQUEST_TIMEOUT_SECS: u64 = 10;
