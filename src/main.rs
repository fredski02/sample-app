mod api;
mod db;
mod kafka;
mod log;
mod middleware;
mod models;
mod services;
mod templates;
mod web;

use axum::Router;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

use crate::api::router as api_router;
use crate::kafka::setup_kafka;
use crate::{db::setup_db, web::router as web_router};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let tracing = log::setup_logging();
    let db = setup_db().await?;
    let event_bus = setup_kafka(db.clone()).await?;

    let web_state = models::state::WebState {
        db: db.clone(),
        events: event_bus.clone(),
    };

    let app = Router::new()
        .merge(web_router())
        .nest("/api", api_router())
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(web_state.clone())
        .route_layer(tracing)
        .layer(middleware::setup_sessions());

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(?addr, "listening");
    axum::serve(listener, app).await?;
    Ok(())
}
