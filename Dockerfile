# Build Stage
FROM rust:1-slim-bookworm as builder

WORKDIR /app

# Create empty shell project to cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
# COPY .sqlx ./.sqlx 

COPY . .
RUN touch src/main.rs

ENV SQLX_OFFLINE=true

RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/JVNMEventStore /app/event-store

EXPOSE 3000
CMD ["./event-store"]