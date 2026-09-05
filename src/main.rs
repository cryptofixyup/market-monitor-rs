use market_monitor_rs::{compute_signal, MarketSnapshot};

fn parse_snapshot(args: &[String]) -> Result<MarketSnapshot, &'static str> {
    if args.len() != 3 {
        return Err("usage: market-monitor-rs <price> <previous_price>");
    }

    let price = args[1].parse::<f64>().map_err(|_| "invalid price")?;
    let previous_price = args[2]
        .parse::<f64>()
        .map_err(|_| "invalid previous_price")?;

    Ok(MarketSnapshot {
        price,
        previous_price,
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let snapshot = match parse_snapshot(&args) {
        Ok(snapshot) => snapshot,
        Err(error) => {
            eprintln!("market-monitor-rs: {error}");
            std::process::exit(2);
        }
    };

    match compute_signal(snapshot) {
        Ok(signal) => println!("market-monitor-rs: signal={signal:?}"),
        Err(error) => {
            eprintln!("market-monitor-rs: signal computation failed: {error:?}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_market_snapshot() {
        let args = vec![
            "market-monitor-rs".to_string(),
            "101.5".to_string(),
            "100.0".to_string(),
        ];

        assert_eq!(
            parse_snapshot(&args),
            Ok(MarketSnapshot {
                price: 101.5,
                previous_price: 100.0,
            })
        );
    }

    #[test]
    fn rejects_wrong_argument_count() {
        let args = vec!["market-monitor-rs".to_string()];
        assert_eq!(
            parse_snapshot(&args),
            Err("usage: market-monitor-rs <price> <previous_price>")
        );
    }

    #[test]
    fn rejects_invalid_price() {
        let args = vec![
            "market-monitor-rs".to_string(),
            "not-a-number".to_string(),
            "100.0".to_string(),
        ];
        assert_eq!(parse_snapshot(&args), Err("invalid price"));
    }
}
