use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use http::StatusCode;

use crate::{
    middleware::AuthedUser,
    models::{
        sample::{Sample, SampleInput},
        state::WebState,
    },
    services,
};

pub fn router() -> Router<WebState> {
    Router::new()
        .route("/samples", get(api_list_samples).post(api_create_sample))
        .route(
            "/samples/{id}",
            get(api_get_sample)
                .patch(api_update_sample)
                .delete(api_delete_sample),
        )
}

async fn api_create_sample(
    State(state): State<WebState>,
    AuthedUser(user_id): AuthedUser,
    Json(input): Json<SampleInput>,
) -> Result<Json<Sample>, StatusCode> {
    let sample = services::sample::create_sample(&state, input, user_id)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(Json(sample))
}

async fn api_list_samples(
    State(state): State<WebState>,
    AuthedUser(_): AuthedUser,
) -> Result<Json<Vec<Sample>>, StatusCode> {
    let samples = services::sample::get_samples(&state).await;
    Ok(Json(samples))
}

async fn api_get_sample(
    State(state): State<WebState>,
    AuthedUser(_): AuthedUser,
    Path(sample_id): Path<i64>,
) -> Result<Json<Sample>, StatusCode> {
    services::sample::get_sample_by_id(&state, &sample_id)
        .await
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn api_update_sample(
    State(state): State<WebState>,
    AuthedUser(_): AuthedUser,
    Path(sample_id): Path<i64>,
    Json(input): Json<SampleInput>,
) -> Result<Json<Sample>, StatusCode> {
    services::sample::update_sample_by_id(&state, input, sample_id)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)
        .map(Json)
}

async fn api_delete_sample(
    State(state): State<WebState>,
    AuthedUser(_): AuthedUser,
    Path(sample_id): Path<i64>,
) -> Result<Json<()>, StatusCode> {
    services::sample::delete_sample_by_id(&state, sample_id)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)
        .map(Json)
}
