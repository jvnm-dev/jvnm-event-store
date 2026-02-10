# JVNM Event Store

A lightweight event store built with Rust. Stores events in PostgreSQL and broadcasts them in real time over WebSocket.

## Functionalities

- **Publish events** via `POST /events` with a topic, event type, and JSON payload
- **Query event history** via `GET /events/:topic` to retrieve all events for a given topic, ordered chronologically
- **Real-time subscriptions** via WebSocket at `/ws?topic=<topic>` (omit `topic` to receive all events)
- **Structured logging** at info and debug levels via `tracing` (controlled by `RUST_LOG` env var)

## Configuration

Environment variables (defined in `.env`):

| Variable       | Description                | Default   |
|----------------|----------------------------|-----------|
| `DATABASE_URL` | PostgreSQL connection URL  | required  |
| `SERVER_PORT`  | HTTP server port           | `3000`    |
| `RUST_LOG`     | Log level filter           | `info`    |

## Running with Docker

### 1. Start the database

```sh
docker compose up -d db
```

### 2. Run migrations

Install the sqlx CLI if you don't have it:

```sh
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

Run the migrations against the running database:

```sh
sqlx migrate run
```

### 3. Prepare offline query data

The Docker build uses `SQLX_OFFLINE=true` so it doesn't need a live database at compile time. You must generate the offline query cache before building the image:

```sh
cargo sqlx prepare
```

This creates a `.sqlx` directory containing cached query metadata. Commit this directory to version control.

### 4. Build and run the application

```sh
docker build -t event_store .
docker run --rm \
  -e DATABASE_URL=postgres://admin:password@host.docker.internal:5432/event_store \
  -p 3000:3000 \
  event_store
```

Or add the app as a service in `docker-compose.yml` and run everything together.

## Running locally (development)

```sh
docker compose up -d db
sqlx migrate run
cargo run
```

## API

### Publish an event

```
POST /events
Content-Type: application/json

{
  "topic": "orders",
  "event_type": "order_created",
  "payload": { "order_id": 42 }
}
```

### Get event history

```
GET /events/orders
```

### Subscribe via WebSocket

```
ws://localhost:3000/ws?topic=orders
```
