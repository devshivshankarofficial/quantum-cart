.PHONY: check test run docker-up docker-down

check:
	cargo check

test:
	cargo test

run:
	cargo run --bin quantum_cart_server

docker-up:
	docker compose up --build

docker-down:
	docker compose down