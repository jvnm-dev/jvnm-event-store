mod domain;
mod infra;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::broadcast;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting event store...");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
        .expect("Fatal DB error");

    tracing::info!("Successfully connected to database");

    let repository = infra::db::postgres::EventRepository::new(pool);
    let (tx, _rx) = broadcast::channel(100);

    let app_state = infra::web::state::AppState {
        repository,
        tx,
    };

    let app = infra::web::routes::create_router(app_state)
        .layer(tower_http::cors::CorsLayer::permissive());

    let port = env::var("SERVER_PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::info!("Event store listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}