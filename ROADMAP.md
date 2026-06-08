# Roadmap

QuantumCart is currently a public-ready experimental backend skeleton. The next work should move it from impressive architecture into a verifiable, benchmarked, production-style system.

## Phase 1: Cloud-Verified Baseline

- Push to GitHub as `quantum-cart`.
- Let GitHub Actions run Rust formatting, check, tests, clippy, and Docker build.
- Fix any CI compiler issues without installing the full Rust toolchain locally.
- Add a CI badge once the repository URL is final.

## Phase 2: Real Event Sourcing

- Replace audit-log-only transaction recording with writes to `event_store`.
- Add event replay for inventory state.
- Add snapshots to avoid replaying long histories.
- Add idempotency keys for checkout and webhooks.

## Phase 3: Security Hardening

- Add login/register endpoints.
- Use Argon2 password hashing.
- Add role-based authorization for buyer, vendor, admin, and auditor roles.
- Add webhook signature verification.
- Set `REQUIRE_AUTH=true` for non-demo deployments.

## Phase 4: Observability

- Add OpenTelemetry traces.
- Export Prometheus metrics.
- Track checkout latency, prediction latency, inventory misses, and checkout conflict rate.
- Add structured trace IDs to event payloads.

## Phase 5: Caching and Rate Limits

- Add Redis read-through product cache.
- Add write-through inventory cache.
- Add Redis-backed token bucket rate limiting.
- Add cache invalidation hooks from order and vendor events.

## Phase 6: Quantum-Inspired Research Layer

- Persist prediction results to `quantum_predictions`.
- Track prediction accuracy against real order volume.
- Add configurable pricing strategies.
- Add benchmarks for basket optimization.

## Phase 7: Public Showcase

- Add architecture screenshots or generated diagrams.
- Add API examples with request and response bodies.
- Add a short demo video or terminal recording.
- Add GitHub Releases once CI is stable.