dev:
	RUST_LOG=info cargo run
build:
	cargo build --release
test:
	cargo test -q
image:
	docker build -t solstice:latest .
run-image:
	docker run --rm -p 8080:8080 -e RUST_LOG=info solstice:latest
fmt:
	cargo fmt
lint:
	cargo clippy -- -D warnings
