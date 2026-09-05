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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rising_price_is_buy() {
        assert_eq!(compute_signal(MarketSnapshot { price: 101.0, previous_price: 100.0 }), Ok(Signal::Buy));
    }

    #[test]
    fn falling_price_is_sell() {
        assert_eq!(compute_signal(MarketSnapshot { price: 99.0, previous_price: 100.0 }), Ok(Signal::Sell));
    }

    #[test]
    fn unchanged_price_is_neutral() {
        assert_eq!(compute_signal(MarketSnapshot { price: 100.0, previous_price: 100.0 }), Ok(Signal::Neutral));
    }

    #[test]
    fn invalid_price_fails_closed() {
        assert_eq!(compute_signal(MarketSnapshot { price: f64::NAN, previous_price: 100.0 }), Err(SignalError::NonFinitePrice));
        assert_eq!(compute_signal(MarketSnapshot { price: 0.0, previous_price: 100.0 }), Err(SignalError::NonPositivePrice));
    }
}
