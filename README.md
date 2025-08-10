Solstice is a production-friendly Rust microservice using Axum, SQLx (SQLite), Utoipa-generated OpenAPI, Prometheus metrics, structured logging with tracing, and graceful shutdown.
Run `cargo run` for local dev. The service exposes `/v1/health`, `/v1/tasks` CRUD, `/metrics`, and `/docs` (Swagger UI).

Quickstart:
1) `rustup default stable && cargo build`
2) `RUST_LOG=info cargo run`
3) Open `http://localhost:8080/docs` for the interactive API.
