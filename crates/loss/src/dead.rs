//! [`DeadSub`]
//!
//! [`DeadSub`]: struct.DeadSub.html
//!
//! The [`DeadSub`] struct implements a [`Subscriber`] that does nothing.
//!
//! ## Examples
//!
//! The simplest way to use [`DeadSub`] is to call [`set_global_default`]:
//!
//! ```rust
//! use loss::dead;
//!
//! dead::set_global_default().unwrap();
//! ```
//!
//! Alternatively, you can use the [`Subscriber`] directly, setting
//! the global default with [`tracing::subscriber::set_global_default`]:
//!
//! ```rust
//! use tracing::Subscriber;
//! use tracing::subscriber::set_global_default;
//! use loss::dead::DeadSub;
//!
//! let subscriber = DeadSub::default();
//! set_global_default(subscriber).unwrap();
//! ```
use tracing::Subscriber;

/// A [`Subscriber`] that does nothing; it is dead.
///
/// [`Subscriber`]: tracing::Subscriber
#[derive(Debug, Default)]
pub struct DeadSub {}

/// A helper function to set the global default [`Subscriber`] to a [`DeadSub`].
pub fn set_global_default() -> anyhow::Result<()> {
    let subscriber = DeadSub::default();
    tracing::subscriber::set_global_default(subscriber).map_err(|e| anyhow::anyhow!(e))
}

impl DeadSub {
    /// Returns a new [`DeadSub`] with the default configuration.
    pub fn new() -> Self {
        Self::default()
    }
}

impl tracing::Subscriber for DeadSub {
    fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {
        false
    }

    fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {
        tracing::span::Id::from_u64(0xDEAD)
    }

    fn record(&self, _: &tracing::span::Id, _: &tracing::span::Record<'_>) {}

    fn record_follows_from(&self, _: &tracing::span::Id, _: &tracing::span::Id) {}

    fn event(&self, _: &tracing::Event<'_>) {}

    fn enter(&self, _: &tracing::span::Id) {}

    fn exit(&self, _: &tracing::span::Id) {}
}
