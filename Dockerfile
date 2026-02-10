FROM rust:1-slim-bookworm as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo update -p home --precise 0.5.11
RUN cargo build --release
RUN rm -rf src

COPY src ./src
COPY .env ./.env
COPY .sqlx ./.sqlx

ENV SQLX_OFFLINE=true

RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/JVNMEventStore /app/event-store

EXPOSE 3000
CMD ["./event-store"]