use axum::{
    extract::{State, Json, Query, Path, ws::{WebSocketUpgrade, WebSocket, Message}},
    response::IntoResponse,
    http::StatusCode,
    routing::{post, get},
    Router,
};
use crate::domain::models::{NewEvent, BroadcastEvent};
use crate::infra::web::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/events", post(publish_event))
        .route("/events/:topic", get(get_events_history))
        .route("/ws", get(websocket_handler))
        .with_state(state)
}

async fn publish_event(
    State(state): State<AppState>,
    Json(payload): Json<NewEvent>,
) -> impl IntoResponse {
    tracing::info!(
        topic = %payload.topic,
        event_type = %payload.event_type,
        "Received request to publish event"
    );

    match state.repository.save(payload).await {
        Ok(saved_event) => {
            tracing::debug!(
                event_id = saved_event.id,
                topic = %saved_event.topic,
                "Event saved successfully, broadcasting to subscribers"
            );

            let broadcast_msg = BroadcastEvent {
                topic: saved_event.topic.clone(),
                event_type: saved_event.event_type.clone(),
                payload: saved_event.payload.clone(),
                occurred_at: saved_event.created_at,
            };
            let _ = state.tx.send(broadcast_msg);
            (StatusCode::CREATED, Json(saved_event)).into_response()
        },
        Err(e) => {
            tracing::error!(error = ?e, "Failed to save event to database");
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save event").into_response()
        }
    }
}

#[derive(serde::Deserialize)]
struct WsParams {
    topic: Option<String>,
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<WsParams>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let topic_label = params.topic.clone().unwrap_or_else(|| "all".to_string());
    tracing::info!(topic = %topic_label, "New WebSocket connection requested");
    ws.on_upgrade(move |socket| handle_socket(socket, state, params.topic))
}

async fn handle_socket(mut socket: WebSocket, state: AppState, filter_topic: Option<String>) {
    let topic_label = filter_topic.clone().unwrap_or_else(|| "all".to_string());
    tracing::debug!(topic = %topic_label, "WebSocket client connected, listening for events");

    let mut rx = state.tx.subscribe();
    while let Ok(msg) = rx.recv().await {
        if let Some(target) = &filter_topic {
            if &msg.topic != target { continue; }
        }
        if let Ok(json_txt) = serde_json::to_string(&msg) {
            tracing::debug!(
                topic = %msg.topic,
                event_type = %msg.event_type,
                "Forwarding event to WebSocket client"
            );
            if socket.send(Message::Text(json_txt)).await.is_err() {
                tracing::info!(topic = %topic_label, "WebSocket client disconnected");
                break;
            }
        }
    }
}

async fn get_events_history(
    State(state): State<AppState>,
    Path(topic): Path<String>,
) -> impl IntoResponse {
    tracing::info!(topic = %topic, "Received request to fetch event history");

    match state.repository.find_by_topic(&topic).await {
        Ok(events) => {
            tracing::debug!(topic = %topic, count = events.len(), "Event history retrieved successfully");
            (StatusCode::OK, Json(events)).into_response()
        },
        Err(e) => {
            tracing::error!(error = ?e, topic = %topic, "Failed to read event history");
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read event history").into_response()
        }
    }
}