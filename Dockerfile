# Multi-stage build for Rust + Axum portfolio
FROM rust:1.82-slim as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
COPY templates ./templates

RUN cargo build --release --bin portfolio

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/target/release/portfolio /app/portfolio
COPY static ./static
COPY templates ./templates

ENV PORT=8080
EXPOSE 8080

CMD ["/app/portfolio"]
