use crate::middleware::{is_htmx, AuthedUser};
use crate::models::sample::SampleInput;
use crate::models::state::WebState;
use crate::services;
use crate::templates::{
    base_ctx, Error403Tmpl, Error404Tmpl, Error500Tmpl, SampleFormTmpl, SamplesListTmpl,
};
use askama::Template;
use axum::response::IntoResponse;
use axum::{
    extract::{Path, State},
    response::{Html, Redirect},
    routing::{get, post},
    Form, Router,
};
use http::header::{CACHE_CONTROL, PRAGMA};
use http::{HeaderMap, HeaderValue, StatusCode};
use serde_json::json;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_sessions::Session;

pub fn router() -> Router<WebState> {
    Router::new()
        .route("/", get(index_page))
        .route("/samples", get(samples_page))
        .route("/samples/new", get(create_page))
        .route("/samples", post(create_sample))
        .route(
            "/samples/{id}",
            get(edit_page).post(update_sample).delete(delete_sample),
        )
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_static("no-store, max-age=0, must-revalidate"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            PRAGMA,
            HeaderValue::from_static("no-cache"),
        ))
}

async fn index_page(AuthedUser(_uid): AuthedUser) -> Redirect {
    Redirect::to("/samples")
}

async fn samples_page(
    State(state): State<WebState>,
    AuthedUser(_uid): AuthedUser,
    session: Session,
) -> Html<String> {
    let ctx = base_ctx(&session).await;
    let samples = services::sample::get_samples(&state).await;
    let html = SamplesListTmpl { ctx, samples }.render().unwrap();
    Html(html)
}

async fn create_page(AuthedUser(_): AuthedUser, session: Session) -> Html<String> {
    let ctx = base_ctx(&session).await;
    let html = SampleFormTmpl {
        ctx,
        s: None,
        action: "/samples".to_string(),
    }
    .render()
    .unwrap();

    Html(html)
}

async fn edit_page(
    State(state): State<WebState>,
    session: Session,
    AuthedUser(user_id): AuthedUser,
    Path(id): Path<i64>,
) -> Html<String> {
    let ctx = base_ctx(&session).await;
    let sample = services::sample::get_sample_by_id(&state, &id).await;

    let html = match sample {
        Some(sample) => {
            if sample.created_by != user_id {
                Error403Tmpl { ctx }.render().unwrap()
            } else {
                SampleFormTmpl {
                    ctx,
                    s: Some(sample),
                    action: format!("/samples/{}", id),
                }
                .render()
                .unwrap()
            }
        }
        None => Error404Tmpl { ctx }.render().unwrap(),
    };

    Html(html)
}

async fn create_sample(
    State(state): State<WebState>,
    headers: HeaderMap,
    session: Session,
    AuthedUser(uid): AuthedUser,
    Form(input): Form<SampleInput>,
) -> impl IntoResponse {
    let ctx = base_ctx(&session).await;
    match services::sample::create_sample(&state, input, uid).await {
        Ok(_) => {
            if is_htmx(&headers) {
                let payload = json!({
                    "path": "/samples",
                    "target": "#shell",
                    "select": "#shell",
                    "swap": "outerHTML swap:200ms",
                    "pushUrl": true
                })
                .to_string();

                let mut hm = HeaderMap::new();
                hm.insert("HX-Location", HeaderValue::from_str(&payload).unwrap());
                (StatusCode::NO_CONTENT, hm).into_response()
            } else {
                Redirect::to("/samples").into_response()
            }
        }
        Err(e) => {
            let html = Error500Tmpl {
                ctx,
                message: e.to_string(),
            }
            .render()
            .unwrap();
            (StatusCode::INTERNAL_SERVER_ERROR, Html(html)).into_response()
        }
    }
}

async fn update_sample(
    State(state): State<WebState>,
    headers: HeaderMap,
    session: Session,
    AuthedUser(_uid): AuthedUser,
    Path(resource_id): Path<i64>,
    Form(input): Form<SampleInput>,
) -> impl IntoResponse {
    let ctx = base_ctx(&session).await;
    match services::sample::update_sample_by_id(&state, input, resource_id).await {
        Ok(_) => {
            if is_htmx(&headers) {
                let payload = json!({
                    "path": "/samples",
                    "target": "#shell",
                    "select": "#shell",
                    "swap": "outerHTML swap:200ms",
                    "pushUrl": true
                })
                .to_string();

                let mut hm = HeaderMap::new();
                hm.insert("HX-Location", HeaderValue::from_str(&payload).unwrap());
                (StatusCode::NO_CONTENT, hm).into_response()
            } else {
                Redirect::to("/samples").into_response()
            }
        }
        Err(e) => {
            let html = Error500Tmpl {
                ctx,
                message: e.to_string(),
            }
            .render()
            .unwrap();
            (StatusCode::INTERNAL_SERVER_ERROR, Html(html)).into_response()
        }
    }
}

async fn delete_sample(
    State(state): State<WebState>,
    session: Session,
    headers: HeaderMap,
    AuthedUser(_): AuthedUser,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let ctx = base_ctx(&session).await;
    match services::sample::delete_sample_by_id(&state, id).await {
        Ok(_) => {
            if is_htmx(&headers) {
                let payload = json!({
                    "path": "/samples",
                    "target": "#shell",
                    "select": "#shell",
                    "swap": "outerHTML swap:200ms",
                    "pushUrl": true
                })
                .to_string();

                let mut hm = HeaderMap::new();
                hm.insert("HX-Location", HeaderValue::from_str(&payload).unwrap());
                (StatusCode::NO_CONTENT, hm).into_response()
            } else {
                Redirect::to("/samples").into_response()
            }
        }
        Err(e) => {
            let html = Error500Tmpl {
                ctx,
                message: e.to_string(),
            }
            .render()
            .unwrap();
            (StatusCode::INTERNAL_SERVER_ERROR, Html(html)).into_response()
        }
    }
}
