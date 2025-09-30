use crate::models::state::WebState;
use axum::Router;
use sample::router as sample_router;

pub mod sample;

pub fn router() -> Router<WebState> {
    Router::new().merge(sample_router())
}
