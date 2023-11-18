use clap::Parser;

/// Run the first stage the axos derivation pipeline.
fn sync_pipe(l2_rpc_url: Option<String>) -> anyhow::Result<()> {
    let l2_rpc_url = l2_rpc_url.ok_or_else(|| anyhow::anyhow!("L2 RPC URL not provided."))?;
    let mock_provider = axos_providers::mock::MockProvider::new(l2_rpc_url);

    let chain_config = axos_primitives::ChainConfig::base_goerli();

    let head = axos::HeadInfoQuery::get_head_info(&mock_provider, &chain_config);

    println!("{:?}", head);

    Ok(())
}

fn main() {
    let axt::cli::Args {
        v,
        l2_rpc_url,
        ..
        // network,
        // l1_rpc_url,
        // sync_mode,
        // l2_engine_url,
        // jwt_secret,
        // rpc_port,
        // checkpoint_hash,
        // checkpoint_sync_url,
    } = axt::cli::Args::parse();
    tracing::debug!(target: "axos", "CLI arguments parsed.");

    if let Err(e) = axt::telemetry::init(v) {
        tracing::error!(target: "axos", "{}", e);
        std::process::exit(1);
    }
    tracing::info!(target: "axos", "Telemtry initialized.");

    if let Err(err) = sync_pipe(l2_rpc_url) {
        tracing::error!(target: "axos", "{}", err);
        std::process::exit(1);
    }
}
