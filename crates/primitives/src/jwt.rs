//! JWT Secret
//!
//! This module was largely built based off of [magi][magi] and [reth][reth].
//!
//! The core JWT signature algorithm uses [jwt_compact][jwt_compact] which is a
//! compact implementation of Json Web Tokens (JWT) with support for no-std.
//!
//! [reth]: https://github.com/paradigmxyz/reth
//! [magi]: https://github.com/a16z/magi

use anyhow::Result;
use jwt_compact::AlgorithmExt;
use rand::rngs::OsRng;
use rand::RngCore;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::claims::Claims;
use crate::GenericString;

/// JWT hex encoded 256 bit secret key length.
const JWT_SECRET_LEN: usize = 64;

/// The JWT signature key for the Hs256 algorithm (HMAC + SHA256).
pub type JwtSignatureKey = jwt_compact::alg::Hs256Key;

/// JwtSecret is a 256-bit hex-encoded secret key used to perform JWT-based authentication.
///
/// See: [Secret key - Engine API specs][engine-api]
///
/// [engine-api]: https://github.com/ethereum/execution-apis/blob/main/src/engine/authentication.md#key-distribution
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub struct JwtSecret([u8; 32]);

impl JwtSecret {
    /// Creates an instance of [`JwtSecret`][JwtSecret].
    /// The provided `secret` must be a valid hexadecimal string of length 64.
    pub fn from_hex<S: AsRef<str>>(hex: S) -> Result<Self> {
        let hex: &str = hex.as_ref().trim();
        if hex.len() != JWT_SECRET_LEN {
            Err(anyhow::anyhow!("Invalid JWT secret key length."))
        } else {
            let hex_bytes = hex::decode(hex).map_err(|e| anyhow::anyhow!(e))?;
            let bytes = hex_bytes.try_into().expect("is expected len");
            Ok(JwtSecret(bytes))
        }
    }

    /// Generates a random [`JwtSecret`][JwtSecret].
    pub fn random() -> Self {
        let mut key = [77u8; 16];
        OsRng.fill_bytes(&mut key);
        let random_bytes = [0u8; 32].map(|_| OsRng.next_u64() as u8);
        let secret = hex::encode(random_bytes);
        JwtSecret::from_hex(secret).unwrap()
    }

    /// Returns if the [`JwtSecret`][JwtSecret] is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns if the provided JWT token is equal to the JWT secret.
    pub fn equal(&self, token: &str) -> bool {
        hex::encode(self.0) == token
    }

    /// Generate claims constructs a [`Claims`][Claims] instance.
    #[allow(dead_code)]
    pub fn generate_claims(dur: chrono::Duration) -> Claims {
        let options_with_stopped_clock =
            jwt_compact::TimeOptions::new(dur, JwtSecret::default_clock());
        Claims::new(&options_with_stopped_clock)
    }

    /// Return the default Clock Function for constructing time options.
    pub fn default_clock() -> impl Fn() -> chrono::DateTime<chrono::Utc> {
        || chrono::DateTime::from_timestamp(60, 0).unwrap()
    }

    /// Grab the [`JwtSignatureKey`][JwtSignatureKey] from the [`JwtSecret`][JwtSecret].
    #[allow(dead_code)]
    pub(crate) fn key(&self) -> JwtSignatureKey {
        JwtSignatureKey::from(&self.0[..])
    }

    /// Encodes the [`Claims`][Claims].
    /// The execution layer client MUST support at least HMAC + SHA256 (Hs256).
    #[allow(dead_code)]
    pub fn encode(&self, claims: Claims) -> anyhow::Result<GenericString> {
        let bytes = &self.0[..];
        let key = JwtSignatureKey::from(bytes);
        let header = jwt_compact::Header::empty().with_token_type("JWT");
        let claims = claims.inner();
        let token_string = jwt_compact::alg::Hs256
            .token(&header, &claims, &key)
            .map_err(|_| anyhow::anyhow!("failed to construct JWT token"))?;
        Ok(token_string)
    }
}

impl core::fmt::Debug for JwtSecret {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JwtSecret").field(&"{{}}").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::claims::InnerClaims;
    use crate::test_utils::{new_mock_claims, new_mock_secret, TEST_ENCODED_TOKEN, TEST_SECRET};

    #[test]
    fn test_encode_jwt_secret() {
        let secret = JwtSecret::from_hex(TEST_SECRET).unwrap();
        let dur = chrono::Duration::seconds(60);
        let claims = JwtSecret::generate_claims(dur);
        let jwt = secret.encode(claims).unwrap();
        assert_eq!(jwt.as_str(), TEST_ENCODED_TOKEN);
    }

    #[test]
    fn test_validate_encoded_jwt_secret() {
        let secret = JwtSecret::from_hex(TEST_SECRET).unwrap();
        let key = secret.key();
        let token = jwt_compact::UntrustedToken::new(TEST_ENCODED_TOKEN).unwrap();
        let validator = jwt_compact::alg::Hs256.validator::<InnerClaims>(&key);
        let validation = validator.validate(&token);
        assert!(validation.is_ok());
    }

    #[test]
    fn test_encode_secret_not_empty() {
        let secret = new_mock_secret().unwrap();
        let claims = new_mock_claims();
        let jwt = secret.encode(claims).unwrap();
        assert!(!jwt.is_empty());
    }

    #[test]
    fn test_random_secret_not_empty() {
        let secret = JwtSecret::random();
        assert!(!secret.is_empty());
        let jwt = secret.encode(new_mock_claims()).unwrap();
        assert!(!jwt.is_empty());
    }

    #[test]
    #[cfg(not(feature = "std"))]
    fn test_valid_mock_claims() {
        let mut claims = new_mock_claims();
        assert!(claims.valid(0));
        claims.iat = 10000000000;
        assert!(!claims.valid(0));
        let max_drift = crate::claims::JWT_MAX_IAT_DIFF.as_seconds_f64() as u64;
        assert!(claims.valid(claims.iat - max_drift));
        assert!(!claims.valid(claims.iat - max_drift - 1));
    }
}
