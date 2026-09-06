use crate::market::{MarketTick, MarketTickError};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MarketSnapshot {
    pub price: f64,
    pub previous_price: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Buy,
    Sell,
    Neutral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalError {
    NonFinitePrice,
    NonPositivePrice,
}

impl From<MarketTickError> for SignalError {
    fn from(error: MarketTickError) -> Self {
        match error {
            MarketTickError::NonFinitePrice => Self::NonFinitePrice,
            MarketTickError::NonPositivePrice => Self::NonPositivePrice,
        }
    }
}

pub fn compute_signal(snapshot: MarketSnapshot) -> Result<Signal, SignalError> {
    if !snapshot.price.is_finite() || !snapshot.previous_price.is_finite() {
        return Err(SignalError::NonFinitePrice);
    }
    if snapshot.price <= 0.0 || snapshot.previous_price <= 0.0 {
        return Err(SignalError::NonPositivePrice);
    }

    Ok(match snapshot.price.total_cmp(&snapshot.previous_price) {
        std::cmp::Ordering::Greater => Signal::Buy,
        std::cmp::Ordering::Less => Signal::Sell,
        std::cmp::Ordering::Equal => Signal::Neutral,
    })
}

pub fn compute_tick_signal(
    previous: MarketTick,
    current: MarketTick,
) -> Result<Signal, SignalError> {
    previous.validate()?;
    current.validate()?;

    compute_signal(MarketSnapshot {
        price: current.price,
        previous_price: previous.price,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rising_price_is_buy() {
        assert_eq!(
            compute_signal(MarketSnapshot {
                price: 101.0,
                previous_price: 100.0,
            }),
            Ok(Signal::Buy)
        );
    }

    #[test]
    fn falling_price_is_sell() {
        assert_eq!(
            compute_signal(MarketSnapshot {
                price: 99.0,
                previous_price: 100.0,
            }),
            Ok(Signal::Sell)
        );
    }

    #[test]
    fn unchanged_price_is_neutral() {
        assert_eq!(
            compute_signal(MarketSnapshot {
                price: 100.0,
                previous_price: 100.0,
            }),
            Ok(Signal::Neutral)
        );
    }

    #[test]
    fn invalid_price_fails_closed() {
        assert_eq!(
            compute_signal(MarketSnapshot {
                price: f64::NAN,
                previous_price: 100.0,
            }),
            Err(SignalError::NonFinitePrice)
        );
        assert_eq!(
            compute_signal(MarketSnapshot {
                price: 0.0,
                previous_price: 100.0,
            }),
            Err(SignalError::NonPositivePrice)
        );
    }

    #[test]
    fn tick_signal_uses_validated_market_data() {
        let previous = MarketTick {
            price: 100.0,
            timestamp_ms: 1_000,
        };
        let current = MarketTick {
            price: 101.0,
            timestamp_ms: 2_000,
        };

        assert_eq!(compute_tick_signal(previous, current), Ok(Signal::Buy));
    }

    #[test]
    fn invalid_tick_fails_closed() {
        let previous = MarketTick {
            price: f64::NAN,
            timestamp_ms: 1_000,
        };
        let current = MarketTick {
            price: 101.0,
            timestamp_ms: 2_000,
        };

        assert_eq!(
            compute_tick_signal(previous, current),
            Err(SignalError::NonFinitePrice)
        );
    }
}
