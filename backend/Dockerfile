# Multi-stage build for QuantumCart Backend
FROM rust:1-slim AS builder

WORKDIR /app
COPY . .
RUN cargo build --release --bin quantum_cart_server

FROM debian:bookworm-slim
LABEL org.opencontainers.image.title="QuantumCart Backend"
LABEL org.opencontainers.image.description="Rust/Axum commerce backend with CQRS and quantum-inspired pricing"

WORKDIR /app
RUN groupadd --system app && useradd --system --gid app --home /app app
COPY --from=builder --chown=app:app /app/target/release/quantum_cart_server /usr/local/bin/
COPY --chown=app:app migrations /app/migrations

ENV RUST_LOG=info
ENV REQUIRE_AUTH=true
USER app
EXPOSE 3000
CMD ["quantum_cart_server"]