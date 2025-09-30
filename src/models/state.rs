use axum::extract::FromRef;
use sqlx::SqlitePool;

use crate::kafka::EventBus;

#[derive(Clone)]
pub struct WebState {
    pub db: SqlitePool,
    pub events: EventBus,
}

impl FromRef<WebState> for SqlitePool {
    fn from_ref(app: &WebState) -> SqlitePool {
        app.db.clone()
    }
}

impl FromRef<WebState> for EventBus {
    fn from_ref(app: &WebState) -> EventBus {
        app.events.clone()
    }
}
