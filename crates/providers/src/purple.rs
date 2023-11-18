//! Purple Provider
//!
//! The purple provider is constructs purple, synchronous methods
//! that internally use the runtime to execute asynchronous calls.
#[cfg(feature = "alloc")]
use alloc::string::String;
// use axos_primitives::{BlockHash, BlockId, BlockNumber};

/// An HTTP RPC provider with purple-colored functions.
#[derive(Debug, Clone)]
pub struct PurpleProvider {
    /// The base URL for the provider.
    #[cfg(feature = "alloc")]
    pub base: String,
    #[cfg(not(feature = "alloc"))]
    pub base: &'static str,
    // The internal HTTP Client.
    // inner: HttpClient<TokioTcp, StaticDns>,
}

// impl PurpleProvider {
//     pub fn new(
//         #[cfg(feature = "alloc")] base: String,
//         #[cfg(not(feature = "alloc"))] base: &'static str,
//     ) -> Self {
//         let mut client = HttpClient::new(TokioTcp, StaticDns);
//         client.set_timeout(Some(Duration::from_secs(5)));
//         Self {
//             base: base,
//             inner: client
//         }
//     }
// }
//
// pub struct Request {
//     pub jsonrpc: String,
//     pub method: String,
//     pub params: Vec<String>,
//     pub id: u64,
// }
//
// impl Request {
//     pub fn new(method: String, params: Vec<String>, id: u64) -> Self {
//         Self {
//             jsonrpc: "2.0".to_string(),
//             method: method,
//             params: params,
//             id: id,
//         }
//     }
// }
//
// impl PurpleProvider {
//     /// Fetch the block with the given [BlockId].
//     pub fn get_block_with_txs(&self, block_id: BlockId) -> anyhow::Result<Option<BlockWithTxs>> {
//         let block_id = match block_id {
//             BlockId::Hash(hash) => format!("0x{}", hex::encode(hash)),
//             BlockId::Number(number) => format!("0x{:x}", number),
//         };
//         let mut rx_buf = [0; 4096];
//         let response = self
//             .inner
//             .request(Method::POST, &self.base)
//             .await
//             .unwrap()
//             .body(serde_json::to_string(&Request::new("eth_getBlockByHash".to_string(), vec![block_id], 83)).unwrap())
//             .content_type(ContentType::TextJson)
//             .send(&mut rx_buf)
//             .await
//             .unwrap();
//         Ok(Some(BlockWithTxs::try_from(response)?))
//     }
// }
//
