# market-monitor-rs

High-performance Rust market monitoring engine for real-time signals, anomaly detection, risk analysis, and scalable financial intelligence.

## Status

Early-stage production foundation.

## Engineering principles

- Rust-first implementation for predictable performance and memory safety.
- Deterministic, testable market-signal computation.
- Fail-closed handling for invalid or incomplete market data.
- Observable pipelines suitable for production deployment.
- Benchmark-driven optimization rather than speculative complexity.

## Signal API

The current signal layer compares the latest valid price with the previous valid price and returns `Buy`, `Sell`, or `Neutral`. Non-finite and non-positive prices are rejected before classification.

## Validation

The CI pipeline enforces formatting, compilation, Clippy warnings, and tests on pushes to `main` and pull requests.

## License

Apache-2.0.
