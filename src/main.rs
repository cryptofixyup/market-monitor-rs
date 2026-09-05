use market_monitor_rs::{compute_signal, MarketSnapshot};

fn main() {
    let snapshot = MarketSnapshot {
        price: 101.0,
        previous_price: 100.0,
    };

    match compute_signal(snapshot) {
        Ok(signal) => println!("market-monitor-rs: signal={signal:?}"),
        Err(error) => {
            eprintln!("market-monitor-rs: signal computation failed: {error:?}");
            std::process::exit(1);
        }
    }
}
