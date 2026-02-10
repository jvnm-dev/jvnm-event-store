use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub topic: String,
    pub event_type: String,
    pub payload: Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewEvent {
    pub topic: String,
    pub event_type: String,
    pub payload: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct BroadcastEvent {
    pub topic: String,
    pub event_type: String,
    pub payload: Value,
    pub occurred_at: DateTime<Utc>,
}