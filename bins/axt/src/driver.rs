//! Driver Configuration

use axos::stages::driver::DriverConfig;

/// Build the driver configuration from the CLI arguments.
pub fn build_driver_config(args: &crate::cli::BuiltArgs) -> DriverConfig {
    DriverConfig {
        l2_rpc_url: args.l2_rpc_url.clone(),
        chain_config: axos_primitives::ChainConfig::base(),
    }
}
