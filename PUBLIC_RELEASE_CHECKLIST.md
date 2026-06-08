# Public Release Checklist

Before making the repository public:

- [ ] Rename the GitHub repository to `quantum-cart`.
- [ ] Confirm `.env` files are not committed.
- [ ] Confirm no private keys, tokens, client secrets, or credentials exist in git history.
- [ ] Push to GitHub and wait for CI.
- [ ] Fix any GitHub Actions failures.
- [ ] Enable Dependabot alerts and security updates.
- [ ] Confirm the weekly `Security / Cargo dependency audit` workflow passes.
- [ ] Enable private vulnerability reporting.
- [ ] Add the final repository URL to README badges if desired.
- [ ] Add topics: `rust`, `axum`, `sqlx`, `postgresql`, `redis`, `cqrs`, `event-sourcing`, `docker`, `kubernetes`, `pricing-engine`.

Recommended GitHub description:

```text
A Rust/Axum commerce backend with CQRS, event-sourcing foundations, PostgreSQL, Redis, Docker, Kubernetes, and a quantum-inspired pricing engine.
```