use tokio::sync::broadcast;
use crate::infra::db::postgres::EventRepository;
use crate::domain::models::BroadcastEvent;

#[derive(Clone)]
pub struct AppState {
    pub repository: EventRepository,
    pub tx: broadcast::Sender<BroadcastEvent>,
}