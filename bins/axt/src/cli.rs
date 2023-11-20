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

/// Non-optional arguments.
#[derive(Debug)]
pub struct BuiltArgs {
    /// The verbosity level.
    pub v: u8,
    /// The network to connect to.
    pub network: String,
    /// The URL of the L1 RPC endpoint.
    pub l1_rpc_url: String,
    /// The URL of the L2 RPC endpoint.
    pub l2_rpc_url: String,
    /// The sync mode to use.
    pub sync_mode: SyncMode,
    /// The URL of the l2 engine api endpoint.
    pub l2_engine_url: String,
    /// A JWT Secret to use for authenticated RPC endpoints.
    pub jwt_secret: String,
    /// The port to listen on for RPC requests.
    pub rpc_port: u16,
    /// The checkpoint hash to sync from.
    pub checkpoint_hash: String,
    /// The checkpoint sync URL to use.
    pub checkpoint_sync_url: String,
}

/// A helper macro that takes an Option<T> and returns the value in anyhow::Result<T>
/// if there is some. if not, it uses the specified environment variable string to try
/// to get the value from the environment, and returns an error if that fails.
macro_rules! ok_or_env {
    ($opt:expr, $env:expr) => {
        $opt.or_else(|| std::env::var($env).ok()).ok_or_else(|| {
            anyhow::anyhow!(concat!(
                "The ",
                stringify!($opt),
                " argument or ",
                stringify!($env),
                " environment variable must be provided."
            ))
        })?
    };
}

impl BuiltArgs {
    /// Parse the arguments from cli and env vars if not specified.
    pub fn parse() -> anyhow::Result<BuiltArgs> {
        let args: Args = Args::parse();
        let l1_rpc_url = ok_or_env!(args.l1_rpc_url, "L1_RPC_URL");
        let l2_rpc_url = ok_or_env!(args.l2_rpc_url, "L2_RPC_URL");
        let l2_engine_url = ok_or_env!(args.l2_engine_url, "L2_ENGINE_URL");
        let checkpoint_hash = ok_or_env!(args.checkpoint_hash, "CHECKPOINT_HASH");
        let checkpoint_sync_url = ok_or_env!(args.checkpoint_sync_url, "CHECKPOINT_SYNC_URL");

        Ok(BuiltArgs {
            v: args.v,
            network: args.network,
            l1_rpc_url,
            l2_rpc_url,
            sync_mode: args.sync_mode,
            l2_engine_url,
            jwt_secret: args.jwt_secret,
            rpc_port: args.rpc_port,
            checkpoint_hash,
            checkpoint_sync_url,
        })
    }
}
