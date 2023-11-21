//! JWT Claims

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "std")]
use chrono::offset::Local;

use jwt_compact::TimeOptions;

/// The inner [Claims][jwt_compact::Claims] type.
pub type InnerClaims = jwt_compact::Claims<jwt_compact::Empty>;

/// Claims are a set of information about an actor authorized by a JWT.
///
/// The Engine API requires that the `iat` (issued-at) claim is provided.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Claims(pub InnerClaims);

/// The maximum amount of drift from the JWT claims issued-at `iat` time.
pub const JWT_MAX_IAT_DIFF: time::Duration = time::Duration::new(60, 0);

impl Claims {
    /// Create a new [Claims] instance with the given issued-at time
    /// and default expiry of 60 seconds.
    pub fn new<F>(issued_at: &TimeOptions<F>) -> Self
    where
        F: Fn() -> chrono::DateTime<chrono::Utc>,
    {
        let inner = jwt_compact::Claims::empty();
        Self(inner.set_duration_and_issuance(
            issued_at,
            chrono::Duration::seconds(JWT_MAX_IAT_DIFF.as_seconds_f64() as i64),
        ))
    }

    /// Consumes [Claims], returning its inner [Claims][jwt_compact::Claims] type.
    pub fn inner(self) -> InnerClaims {
        self.0
    }

    /// Valid returns if the given claims are valid against
    /// the provided timestamp in seconds from the UNIX_EPOCH.
    #[cfg(not(feature = "std"))]
    #[allow(dead_code)]
    pub fn valid(&self, now: u64) -> bool {
        let issued_at = if let Some(d) = self.0.issued_at {
            d.timestamp() as u64
        } else {
            return false;
        };
        now.abs_diff(issued_at) <= (JWT_MAX_IAT_DIFF.as_seconds_f64() as u64)
    }

    /// Valid returns if the given claims are valid.
    #[cfg(feature = "std")]
    #[allow(dead_code)]
    pub fn valid(&self) -> bool {
        let now = Local::now();
        let now_secs = now.timestamp();
        let issued_at = if let Some(d) = self.0.issued_at {
            d.timestamp()
        } else {
            return false;
        };
        now_secs.abs_diff(issued_at) <= (JWT_MAX_IAT_DIFF.as_seconds_f64() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(not(feature = "std"))]
    fn test_construct_claims() {
        let dur = chrono::Duration::seconds(60);
        let clock = || chrono::DateTime::from_timestamp(60, 0).unwrap();
        let to = jwt_compact::TimeOptions::new(dur, clock);
        let claims = Claims::new(&to);
        assert!(claims.valid(0));
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_construct_claims_with_now() {
        let dur = chrono::Duration::seconds(60);
        let clock = || chrono::Utc::now();
        let to = jwt_compact::TimeOptions::new(dur, clock);
        let claims = Claims::new(&to);
        assert!(claims.valid());
    }
}
