/// Run the first stage the axos derivation pipeline.
fn sync_pipe(args: axt::cli::BuiltArgs) -> anyhow::Result<()> {
    // First Stage: driver
    let config = axt::driver::build_driver_config(&args);
    let first_stage = axos::stages::driver::Driver::from(config);
    println!("Built first stage: {:?}", first_stage);

    Ok(())
}

fn main() {
    let args = axt::cli::BuiltArgs::parse().unwrap_or_else(|e| {
        tracing::error!(target: "axos", "{}", e);
        std::process::exit(1);
    });
    tracing::debug!(target: "axos", "CLI arguments parsed.");

    if let Err(e) = axt::telemetry::init(args.v) {
        tracing::error!(target: "axos", "{}", e);
        std::process::exit(1);
    }
    tracing::info!(target: "axos", "Telemtry initialized.");

    if let Err(err) = sync_pipe(args) {
        tracing::error!(target: "axos", "{}", err);
        std::process::exit(1);
    }
}
