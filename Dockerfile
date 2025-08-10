# Build
FROM rust:1.79 as builder
WORKDIR /app
COPY Cargo.toml .
RUN mkdir -p src && echo "fn main(){}" > src/main.rs && cargo build --release
COPY . .
RUN cargo build --release

# Run (distroless-ish slim)
FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /app/target/release/solstice /app/solstice
ENV RUST_LOG=info
EXPOSE 8080
USER 65532:65532
ENTRYPOINT ["/app/solstice"]
