# QuantumCart

<p align="center">
  <strong>A Rust commerce backend exploring CQRS, event sourcing, and quantum-inspired optimization.</strong>
</p>

<p align="center">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white">
  <img alt="Axum" src="https://img.shields.io/badge/Axum-111827?style=for-the-badge">
  <img alt="PostgreSQL" src="https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white">
  <img alt="Redis" src="https://img.shields.io/badge/Redis-DC382D?style=for-the-badge&logo=redis&logoColor=white">
  <img alt="Docker" src="https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white">
  <img alt="Kubernetes" src="https://img.shields.io/badge/Kubernetes-326CE5?style=for-the-badge&logo=kubernetes&logoColor=white">
</p>

---

## What Is QuantumCart?

QuantumCart is an experimental, production-oriented e-commerce backend built with **Rust**, **Axum**, **PostgreSQL**, and **Redis**. It combines practical backend architecture with a research-style pricing layer that uses **quantum-inspired optimization** for demand forecasting, dynamic pricing, and cart-level strategy simulation.

This is not a real quantum-computing system. The "quantum" layer is intentionally described as **quantum-inspired**: Monte Carlo sampling, matrix perturbation, bounded optimization, and probabilistic confidence scoring. That makes the project ambitious while staying technically honest.

## Why It Exists

Most commerce demos stop at CRUD. QuantumCart is designed to show something deeper:

- How a modern Rust backend can be structured for scale.
- How CQRS and event-sourcing patterns can shape checkout flows.
- How pricing and inventory can become intelligent systems rather than static tables.
- How a backend can be prepared for Docker, Kubernetes, observability, and public open-source presentation from day one.

## System Shape

```mermaid
flowchart LR
    Client[Client / Storefront] --> API[Axum API]
    API --> Auth[JWT / Zero-Trust Layer]
    API --> CQRS[CQRS Command + Query Flow]
    CQRS --> Orders[Order Context]
    CQRS --> Products[Product + Inventory Queries]
    Orders --> Events[Event Store / Audit Log]
    Products --> Postgres[(PostgreSQL)]
    Products --> Redis[(Redis Cache)]
    Orders --> Postgres
    API --> Quantum[Quantum-Inspired Pricing Engine]
    Quantum --> Predictions[(Prediction History)]
```

## Core Capabilities

| Area | What It Does |
|---|---|
| API | Rust/Axum HTTP server with health, products, inventory, checkout, webhooks, and prediction routes |
| CQRS | Checkout flow separated into command-style order handling |
| Event Foundation | Transaction completion events and audit/event persistence hooks |
| Database | PostgreSQL schema with vendors, products, inventory, partitioned transactions, audit logs, users, events, and quantum predictions |
| Cache | Redis client foundation for inventory/product caching and rate limiting |
| Quantum Engine | Demand prediction, dynamic pricing, basket optimization, and confidence scoring |
| Security | JWT middleware with optional strict auth mode via `REQUIRE_AUTH=true` |
| Deployment | Dockerfile, Docker Compose, Kubernetes Deployment, Service, and HPA manifests |

## Repository Layout

```text
.
├── ARCHITECTURE.md
├── README.md
└── backend/
    ├── Cargo.toml
    ├── Dockerfile
    ├── docker-compose.yml
    ├── Makefile
    ├── README.md
    ├── migrations/
    │   └── init.sql
    ├── k8s/
    │   ├── deployment.yaml
    │   ├── service.yaml
    │   └── hpa.yaml
    └── src/
        ├── main.rs
        ├── lib.rs
        ├── shared/
        ├── domains/
        │   ├── pricing/
        │   ├── order/
        │   ├── cart/
        │   ├── vendor/
        │   ├── identity/
        │   └── audit/
        ├── application/
        └── infrastructure/
```

## API Surface

| Method | Endpoint | Purpose |
|---|---|---|
| `GET` | `/health` | Runtime health and readiness signal |
| `GET` | `/api/v1/products` | List product catalog entries |
| `GET` | `/api/v1/products/{id}` | Get one product |
| `GET` | `/api/v1/inventory/{product_id}` | Get inventory, with quantum fallback if DB is unavailable |
| `POST` | `/api/v1/checkout` | Execute checkout command |
| `POST` | `/api/v1/plugin/import` | Receive plugin/import payloads |
| `POST` | `/api/v1/webhooks/vendor` | Receive vendor inventory/order updates |
| `GET` | `/api/v1/quantum/predict/{product_id}` | Generate demand and price prediction |

## Run Locally

If your laptop has limited storage, you do not need to install Rust locally first. Push the repo to GitHub and let **GitHub Actions** run `cargo fmt`, `cargo check`, `cargo test`, `cargo clippy`, and Docker image builds in the cloud.

### Option 1: Full Stack With Docker

```powershell
cd backend
docker compose up --build
```

The API will run at:

```text
http://localhost:3000
```

Health check:

```powershell
Invoke-RestMethod http://localhost:3000/health
```

Quantum prediction:

```powershell
Invoke-RestMethod http://localhost:3000/api/v1/quantum/predict/00000000-0000-0000-0000-000000000001
```

### Option 2: Native Rust

Install Rust from https://rustup.rs, then:

```powershell
cd backend
cargo check
cargo test
cargo run --bin quantum_cart_server
```

## Environment

Create `backend/.env` from `backend/.env.example`:

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/quantumcart
REDIS_URL=redis://127.0.0.1:6379/
JWT_SECRET=change-me-in-production
REQUIRE_AUTH=false
RUST_LOG=info
```

Set `REQUIRE_AUTH=true` when you want mutating endpoints to require JWT Bearer tokens.

## Quantum-Inspired Pricing

The pricing engine currently includes:

- Monte Carlo demand forecasting.
- Matrix perturbation for uncertainty simulation.
- Price elasticity guardrails.
- Basket-level optimization using `nalgebra`.
- Confidence scoring for generated predictions.

Example response shape:

```json
{
  "product_id": "00000000-0000-0000-0000-000000000001",
  "quantum_predicted_demand": 143,
  "optimized_dynamic_price": 50.98,
  "quantum_confidence": 0.93,
  "algorithm": "monte-carlo-superposition + matrix-perturbation + basket-annealing"
}
```

## Production Direction

QuantumCart is ready to evolve into a deeper commerce platform with:

- Real event-store replay and snapshots.
- Redpanda/Kafka event bus integration.
- OpenTelemetry traces and Prometheus metrics.
- JWT login/register and role-based access control.
- Product import pipeline for external commerce systems.
- Benchmark suite for checkout and prediction latency.
- CI/CD with GitHub Actions.

## Low-Space Development Workflow

For machines without enough disk space for Rust toolchains and `target/` builds:

1. Make source changes locally.
2. Commit and push to GitHub.
3. Let GitHub Actions run Rust checks and Docker builds.
4. Fix only the errors reported by CI.
5. Use Docker or a cloud dev box for full runtime testing.

This keeps the local repository tiny while still giving the project real verification.

## Open-Source Inspiration

QuantumCart follows patterns commonly found in strong Rust backend projects and templates:

- Axum + SQLx + PostgreSQL service structure.
- Docker Compose for local infrastructure.
- GitHub Actions with rustfmt, clippy, test, and cache.
- Public repo hygiene: MIT license, SECURITY.md, CONTRIBUTING.md, issue templates, Dependabot.
- Kubernetes readiness and horizontal scaling manifests.

## Public Repo Positioning

Recommended GitHub repository name:

```text
quantum-cart
```

Recommended description:

```text
A Rust/Axum commerce backend with CQRS, event-sourcing foundations, PostgreSQL, Redis, Docker, Kubernetes, and a quantum-inspired pricing engine.
```

## Current Status

- Static editor diagnostics: clean.
- Native `cargo check`: pending, because Rust/Cargo is not currently installed or not on PATH in the local environment.
- Docker-based build should work once Docker can pull the Rust image and compile dependencies.

## License

MIT. See [LICENSE](LICENSE).

---

<p align="center">
  Built as a serious systems architecture experiment: practical backend engineering, research-inspired optimization, and deployment-first thinking.
</p>
