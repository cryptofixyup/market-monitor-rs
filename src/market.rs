#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MarketTick {
    pub price: f64,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarketTickError {
    NonFinitePrice,
    NonPositivePrice,
}

impl MarketTick {
    pub fn validate(self) -> Result<Self, MarketTickError> {
        if !self.price.is_finite() {
            return Err(MarketTickError::NonFinitePrice);
        }
        if self.price <= 0.0 {
            return Err(MarketTickError::NonPositivePrice);
        }
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_tick() {
        let tick = MarketTick {
            price: 101.5,
            timestamp_ms: 1_700_000_000_000,
        };
        assert_eq!(tick.validate(), Ok(tick));
    }

    #[test]
    fn rejects_non_finite_price() {
        let tick = MarketTick {
            price: f64::NAN,
            timestamp_ms: 1,
        };
        assert_eq!(tick.validate(), Err(MarketTickError::NonFinitePrice));
    }

    #[test]
    fn rejects_non_positive_price() {
        let tick = MarketTick {
            price: 0.0,
            timestamp_ms: 1,
        };
        assert_eq!(tick.validate(), Err(MarketTickError::NonPositivePrice));
    }
}
