pub mod market;
pub mod signal;

pub use market::{MarketTick, MarketTickError};
pub use signal::{compute_signal, MarketSnapshot, Signal, SignalError};
