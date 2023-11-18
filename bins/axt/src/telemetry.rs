//! Telemetry module for axt.

/// Initializes telemetry with the given verbosity level.
pub fn init(verbosity_level: u8) -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(match verbosity_level {
            0 => tracing::Level::ERROR,
            1 => tracing::Level::WARN,
            2 => tracing::Level::INFO,
            3 => tracing::Level::DEBUG,
            _ => tracing::Level::TRACE,
        })
        .finish();
    tracing::subscriber::set_global_default(subscriber).map_err(|e| anyhow::anyhow!(e))
}
