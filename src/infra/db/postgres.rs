use sqlx::PgPool;
use crate::domain::models::{Event, NewEvent};

#[derive(Clone)]
pub struct EventRepository {
    pool: PgPool,
}

impl EventRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, new_event: NewEvent) -> Result<Event, sqlx::Error> {
        tracing::debug!(
            topic = %new_event.topic,
            event_type = %new_event.event_type,
            "Inserting new event into database"
        );

        let event = sqlx::query_as!(
            Event,
            r#"
            INSERT INTO events (topic, event_type, payload)
            VALUES ($1, $2, $3)
            RETURNING id, topic, event_type, payload, created_at
            "#,
            new_event.topic,
            new_event.event_type,
            new_event.payload
        )
            .fetch_one(&self.pool)
            .await?;

        tracing::debug!(event_id = event.id, "Event inserted successfully");
        Ok(event)
    }

    pub async fn find_by_topic(&self, topic: &str) -> Result<Vec<Event>, sqlx::Error> {
        tracing::debug!(topic = %topic, "Querying events by topic");

        let events = sqlx::query_as!(
            Event,
            "SELECT * FROM events WHERE topic = $1 ORDER BY created_at ASC",
            topic
        )
            .fetch_all(&self.pool)
            .await?;

        tracing::debug!(topic = %topic, count = events.len(), "Query completed");
        Ok(events)
    }
}