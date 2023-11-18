//! Command line interface arguments.
use axos_primitives::SyncMode;
use clap::{ArgAction, Parser};
use serde::Serialize;

/// Cli arguments.
#[derive(Debug, Parser, Serialize)]
pub struct Args {
    /// Verbosity level (0-4). Default: 0 (ERROR).
    #[arg(long, short, action = ArgAction::Count, default_value = "0")]
    pub v: u8,

    /// The network to connect to. Default: base-goerli.
    #[clap(short, long, default_value = "base-goerli")]
    pub network: String,

    /// The URL of the L1 RPC endpoint.
    /// Example format: https://eth-goerli.g.alchemy.com/v2/<API_KEY>
    #[clap(long)]
    pub l1_rpc_url: Option<String>,

    /// The URL of the L2 RPC endpoint.
    /// Example format: https://opt-goerli.g.alchemy.com/v2/<API_KEY>
    #[clap(long)]
    pub l2_rpc_url: Option<String>,

    /// The sync mode to use. Default: full.
    #[clap(short = 'm', long, default_value = "full")]
    pub sync_mode: SyncMode,

    /// The URL of the l2 engine api endpoint.
    #[clap(long)]
    pub l2_engine_url: Option<String>,

    /// A JWT Secret to use for authenticated RPC endpoints.
    #[clap(
        long,
        default_value = "bf549f5188556ce0951048ef467ec93067bc4ea21acebe46ef675cd4e8e015ff"
    )]
    pub jwt_secret: String,

    /// The port to listen on for RPC requests. Default: 9545.
    #[clap(short = 'p', long, default_value = "9545")]
    pub rpc_port: u16,

    /// The checkpoint hash to sync from.
    #[clap(long)]
    pub checkpoint_hash: Option<String>,

    /// The checkpoint sync URL to use.
    #[clap(long)]
    pub checkpoint_sync_url: Option<String>,
}
