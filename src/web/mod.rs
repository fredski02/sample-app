use axum::Router;

use crate::models::state::WebState;
use auth::router as auth_router;
use sample::router as sample_router;

pub mod auth;
pub mod sample;

pub fn router() -> Router<WebState> {
    Router::new().merge(auth_router()).merge(sample_router())
}
