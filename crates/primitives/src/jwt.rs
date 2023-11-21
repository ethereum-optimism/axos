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
use chrono::{offset::Local, DateTime};

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

    /// Returns if the provided JWT token is equal to the JWT secret.
    pub fn equal(&self, token: &str) -> bool {
        hex::encode(self.0) == token
    }

    /// Generate claims constructs a [`Claims`][crate::engine::Claims] instance.
    ///
    /// ## Panics
    ///
    /// This function will panic if the system time is before the UNIX_EPOCH.
    #[cfg(feature = "std")]
    #[allow(dead_code)]
    pub(crate) fn generate_claims(&self, time: Option<DateTime<Local>>) -> Claims {
        let now = time.unwrap_or_else(Local::now);
        let now_secs = now.timestamp();
        Claims {
            iat: now_secs as u64,
            exp: (now_secs + 60) as u64,
        }
    }

    /// Encodes the [`Claims`][crate::engine::Claims] in a [jsonwebtoken::Header] String format.
    /// The execution layer client MUST support at least the following alg HMAC + SHA256 (HS256).
    #[allow(dead_code)]
    pub(crate) fn encode(&self, claims: &Claims) -> anyhow::Result<GenericString> {
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
pub(crate) struct Claims {
    /// The number of seconds since the UNIX_EPOCH.
    pub(crate) iat: u64,
    /// The expiration time of the JWT.
    pub(crate) exp: u64,
}

/// The maximum amount of drift from the JWT claims issued-at `iat` time.
#[cfg(feature = "std")]
#[allow(dead_code)]
const JWT_MAX_IAT_DIFF: time::Duration = time::Duration::new(60, 0);

#[cfg(feature = "std")]
impl Claims {
    /// Valid returns if the given claims are valid.
    #[allow(dead_code)]
    pub(crate) fn valid(&self) -> bool {
        let now = Local::now();
        let now_secs = now.timestamp();
        now_secs.abs_diff(self.iat as i64) <= (JWT_MAX_IAT_DIFF.as_seconds_f64() as u64)
    }
}
