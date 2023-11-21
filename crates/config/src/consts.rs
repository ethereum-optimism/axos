//! Constants
//!
//! This module contains constants used in the axos derivation pipeline.

/// The default engine api authentication port
pub const DEFAULT_AUTH_PORT: u16 = 8551;

/// The ID of the static payload
pub const STATIC_ID: u32 = 1;

/// The json rpc version string
pub const JSONRPC_VERSION: &str = "2.0";

/// The new payload method string
pub const ENGINE_NEW_PAYLOAD_V1: &str = "engine_newPayloadV1";

/// Version 2 engine new payload method string
pub const ENGINE_NEW_PAYLOAD_V2: &str = "engine_newPayloadV2";

/// The new payload timeout
pub const ENGINE_NEW_PAYLOAD_TIMEOUT: time::Duration = time::Duration::new(8, 0);

/// The get payload method string
pub const ENGINE_GET_PAYLOAD_V1: &str = "engine_getPayloadV1";

/// Version 2 engine get payload method string
pub const ENGINE_GET_PAYLOAD_V2: &str = "engine_getPayloadV2";

/// The get payload timeout
pub const ENGINE_GET_PAYLOAD_TIMEOUT: time::Duration = time::Duration::new(2, 0);

/// The forkchoice updated method string
pub const ENGINE_FORKCHOICE_UPDATED_V1: &str = "engine_forkchoiceUpdatedV1";

/// Version 2 engine forkchoice updated method string
pub const ENGINE_FORKCHOICE_UPDATED_V2: &str = "engine_forkchoiceUpdatedV2";

/// The forkchoice updated timeout
pub const ENGINE_FORKCHOICE_UPDATED_TIMEOUT: time::Duration = time::Duration::new(8, 0);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_payload_timeout() {
        assert_eq!(ENGINE_NEW_PAYLOAD_TIMEOUT.as_seconds_f64(), 8_f64);
        assert_eq!(
            ENGINE_NEW_PAYLOAD_TIMEOUT.whole_nanoseconds(),
            8000000000_i128
        );
    }

    #[test]
    fn test_engine_get_payload_timeout() {
        assert_eq!(ENGINE_GET_PAYLOAD_TIMEOUT.as_seconds_f64(), 2_f64);
        assert_eq!(
            ENGINE_GET_PAYLOAD_TIMEOUT.whole_nanoseconds(),
            2000000000_i128
        );
    }

    #[test]
    fn test_engine_forkchoice_updated_timeout() {
        assert_eq!(ENGINE_FORKCHOICE_UPDATED_TIMEOUT.as_seconds_f64(), 8_f64);
        assert_eq!(
            ENGINE_FORKCHOICE_UPDATED_TIMEOUT.whole_nanoseconds(),
            8000000000_i128
        );
    }
}
