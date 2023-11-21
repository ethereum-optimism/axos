//! Test utilities for the primitives crate.
//!
//! This module is only available when the `test-utils` feature is enabled.

use anyhow::Result;

use crate::claims::Claims;
use crate::jwt::JwtSecret;

/// Test JWT Secret
pub const TEST_SECRET: &str = "f79ae8046bc11c9927afe911db7143c51a806c4a537cc08e0d37140b0192f430";

/// Test encoded JWT Token
/// Built using claims issued at the unix epoch with the default 60 second expiry.
pub const TEST_ENCODED_TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjEyMCwiaWF0Ijo2MH0.8wPOSzmSa-atOmdob8IW3plAxYsfynurHRXx_Y0gtCE";

/// Creates mock JWT [Claims] for testing.
pub fn new_mock_claims() -> Claims {
    JwtSecret::generate_claims(chrono::Duration::seconds(60))
}

/// Creates a mock JWT Secret for testing.
pub fn new_mock_secret() -> Result<JwtSecret> {
    JwtSecret::from_hex(TEST_SECRET)
}
