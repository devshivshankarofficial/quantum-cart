# Contributing

Thanks for helping improve QuantumCart.

## Development Checks

Run these from `backend/` when Rust is available:

```powershell
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
```

If your local machine does not have enough disk space for Rust builds, push your branch and let GitHub Actions run the checks in the cloud.

## Contribution Style

- Keep the quantum layer honest: describe it as quantum-inspired unless real quantum hardware or quantum SDK integration exists.
- Prefer small, reviewable pull requests.
- Add tests for pricing, checkout, and persistence changes.
- Avoid committing secrets, `.env` files, build artifacts, or generated binaries.

## Good First Areas

- Event-store replay and snapshots.
- JWT login/register endpoints.
- OpenTelemetry and Prometheus metrics.
- Redis-backed caching and rate limiting.
- Product import pipeline.
- Benchmark suite for checkout and prediction latency.