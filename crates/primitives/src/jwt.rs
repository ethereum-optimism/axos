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

#[cfg(feature = "std")]
use chrono::offset::Local;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
#[derive(Clone)]
pub struct JwtSecret([u8; 32]);

impl JwtSecret {
    /// Creates an instance of [`JwtSecret`][crate::jwt::JwtSecret].
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

    /// Generates a random [`JwtSecret`][crate::jwt::JwtSecret].
    pub fn random() -> Self {
        let mut key = [77u8; 16];
        OsRng.fill_bytes(&mut key);
        let random_bytes = [0u8; 32].map(|_| OsRng.next_u64() as u8);
        let secret = hex::encode(random_bytes);
        JwtSecret::from_hex(secret).unwrap()
    }

    /// Returns if the [`JwtSecret`][crate::jwt::JwtSecret] is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns if the provided JWT token is equal to the JWT secret.
    pub fn equal(&self, token: &str) -> bool {
        hex::encode(self.0) == token
    }

    /// Generate claims constructs a [`Claims`][crate::engine::Claims] instance.
    #[allow(dead_code)]
    pub fn generate_claims(t: time::OffsetDateTime) -> Claims {
        let now_secs = t.unix_timestamp() as u64;
        Claims {
            iat: now_secs,
            exp: now_secs + 60,
        }
    }

    /// Encodes the [`Claims`][crate::engine::Claims].
    /// The execution layer client MUST support at least HMAC + SHA256 (Hs256).
    #[allow(dead_code)]
    pub fn encode(&self, claims: &Claims) -> anyhow::Result<GenericString> {
        let bytes = &self.0[..];
        let key = JwtSignatureKey::from(bytes);
        let header = jwt_compact::Header::empty().with_key_id("axos");
        let claims = jwt_compact::Claims::new(claims);
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

/// Claims are a set of information about an actor authorized by a JWT.
///
/// The Engine API requires that the `iat` (issued-at) claim is provided.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Claims {
    /// The number of seconds since the UNIX_EPOCH.
    pub iat: u64,
    /// The expiration time of the JWT.
    pub exp: u64,
}

/// The maximum amount of drift from the JWT claims issued-at `iat` time.
const JWT_MAX_IAT_DIFF: time::Duration = time::Duration::new(60, 0);

impl Claims {
    /// Valid returns if the given claims are valid against
    /// the provided timestamp in seconds from the UNIX_EPOCH.
    #[cfg(not(feature = "std"))]
    #[allow(dead_code)]
    pub fn valid(&self, now: u64) -> bool {
        now.abs_diff(self.iat) <= (JWT_MAX_IAT_DIFF.as_seconds_f64() as u64)
    }

    /// Valid returns if the given claims are valid.
    #[cfg(feature = "std")]
    #[allow(dead_code)]
    pub fn valid(&self) -> bool {
        let now = Local::now();
        let now_secs = now.timestamp();
        now_secs.abs_diff(self.iat as i64) <= (JWT_MAX_IAT_DIFF.as_seconds_f64() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{new_mock_claims, new_mock_secret};

    #[test]
    fn test_encode_secret() {
        let secret = new_mock_secret().unwrap();
        let claims = new_mock_claims();
        let jwt = secret.encode(&claims).unwrap();
        assert!(!jwt.is_empty());
    }

    #[test]
    fn test_random_secret_not_empty() {
        let secret = JwtSecret::random();
        assert!(!secret.is_empty());
        let jwt = secret.encode(&new_mock_claims()).unwrap();
        assert!(!jwt.is_empty());
    }

    #[test]
    #[cfg(not(feature = "std"))]
    fn test_valid_mock_claims() {
        let mut claims = new_mock_claims();
        assert!(claims.valid(0));
        claims.iat = 10000000000;
        assert!(!claims.valid(0));
        let max_drift = JWT_MAX_IAT_DIFF.as_seconds_f64() as u64;
        assert(claims.valid(claims.iat - max_drift));
        assert(!claims.valid(claims.iat - max_drift - 1));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_construct_valid_secret_claims() {
        let c = JwtSecret::generate_claims(std::time::SystemTime::now().into());
        assert!(c.valid());
    }
}
