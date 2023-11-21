//! Engine API

use axos_config::consts::DEFAULT_AUTH_PORT;
use axos_primitives::jwt::JwtSecret;
use axos_primitives::GenericString;

use anyhow::Result;

#[cfg(feature = "alloc")]
use alloc::format;

/// The Engine Api
#[derive(Debug, Clone)]
pub struct EngineApi {
    /// Base request url
    pub base_url: GenericString,
    /// The url port
    pub port: u16,
    /// Internal [JwtSecret][axos_primitives::jwt::JwtSecret] used to authenticate with the engine api.
    #[allow(dead_code)]
    secret: JwtSecret,
}

impl EngineApi {
    /// Create a new [EngineApi][crate::engine::EngineApi] instance.
    pub fn new(base_url: GenericString, secret_str: &str) -> Self {
        let secret = JwtSecret::from_hex(secret_str).unwrap_or_else(|_|
            panic!(
                "Invalid JWT secret. \
                Must be a 256 bit hex-encoded secret key used to authenticate with the engine api. \
                This should be the same as set in the `--auth.secret` flag when executing go-ethereum."
            )
        );
        let (base_url, port) = Self::split_base_url(base_url).unwrap_or_else(|_| {
            panic!(
                "Invalid base url. \
                Must be of the form \"http://<addr>:<port>\""
            )
        });
        Self {
            base_url,
            port,
            secret,
        }
    }

    /// Splits the base url into the address and port
    fn split_base_url(base_url: impl Into<GenericString>) -> Result<(GenericString, u16)> {
        let binding = base_url.into();
        let prefix = binding.split_once("http://").map(|_| "http://").unwrap_or(
            binding
                .split_once("https://")
                .map(|_| "https://")
                .unwrap_or(""),
        );
        let binding = binding.strip_prefix(prefix).unwrap_or(&binding);

        // Split the binding into the address and port
        let mut binding = binding.split(':');
        let addr = binding.next().ok_or_else(|| {
            anyhow::anyhow!(
                "Invalid base url. \
            Must be of the form \"http://<addr>:<port>\""
            )
        })?;
        if addr.is_empty() {
            return Err(anyhow::anyhow!(
                "Empty addr. Must be of the form \"http://<addr>:<port>\""
            ));
        }
        let port = binding.next().unwrap_or("8551");
        let port = port
            .parse::<u16>()
            .map_err(|_| anyhow::anyhow!("Invalid port in base url. Must be a valid u16"))?;
        let addr = format!("{}{}:{}", prefix, addr, port);
        Ok((addr, port))
    }

    /// Constructs the base engine api url for the given address
    pub fn auth_url_from_addr(addr: &str, port: Option<u16>) -> GenericString {
        let stripped = addr.strip_prefix("http://").unwrap_or(addr);
        let stripped = addr.strip_prefix("https://").unwrap_or(stripped);
        let port = port.unwrap_or(DEFAULT_AUTH_PORT);
        GenericString::from(format!("http://{stripped}:{port}"))
    }
}

#[cfg(feature = "std")]
impl EngineApi {
    /// Creates an engine api from environment variables
    pub fn from_env() -> Self {
        let base_url = std::env::var("ENGINE_API_URL").unwrap_or_else(|_| {
            panic!(
                "ENGINE_API_URL environment variable not set. \
                Please set this to the base url of the engine api"
            )
        });
        let secret_key = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
            panic!(
                "JWT_SECRET environment variable not set. \
                Please set this to the 256 bit hex-encoded secret key used to authenticate with the engine api. \
                This should be the same as set in the `--auth.secret` flag when executing go-ethereum."
            )
        });
        let base_url = EngineApi::auth_url_from_addr(&base_url, None);
        Self::new(base_url, &secret_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const AUTH_ADDR: &str = "0.0.0.0";
    const SECRET: &str = "f79ae8046bc11c9927afe911db7143c51a806c4a537cc08e0d37140b0192f430";

    #[test]
    fn test_engine_api_construction() {
        let base_url = EngineApi::auth_url_from_addr(AUTH_ADDR, Some(8551));
        assert_eq!(base_url, "http://0.0.0.0:8551");
        let engine_api = EngineApi::new(base_url, SECRET);
        assert_eq!(engine_api.base_url, "http://0.0.0.0:8551");
        assert_eq!(engine_api.port, 8551);
    }

    #[test]
    fn test_engine_api_split_http() {
        let base_url = "http://localhost:8551";
        let (addr, port) = EngineApi::split_base_url(base_url).unwrap();
        assert_eq!(addr, "http://localhost:8551");
        assert_eq!(port, 8551);
    }

    #[test]
    fn test_engine_api_split_https() {
        let base_url = "https://localhost";
        let (addr, port) = EngineApi::split_base_url(base_url).unwrap();
        assert_eq!(addr, "https://localhost:8551");
        assert_eq!(port, 8551);
    }

    #[test]
    fn test_engine_api_split_missing_port() {
        let base_url = "http://localhost";
        let (addr, port) = EngineApi::split_base_url(base_url).unwrap();
        assert_eq!(addr, "http://localhost:8551");
        assert_eq!(port, 8551);
    }

    #[test]
    fn test_engine_api_split_empty() {
        let base_url = "";
        let res = EngineApi::split_base_url(base_url);
        assert!(res.is_err());
    }

    #[test]
    fn test_engine_api_split_colon() {
        let base_url = ":";
        let res = EngineApi::split_base_url(base_url);
        assert!(res.is_err());
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_engine_api_from_env() {
        std::env::set_var("ENGINE_API_URL", "localhost");
        std::env::set_var("JWT_SECRET", SECRET);
        let _ = EngineApi::from_env();
    }
}
