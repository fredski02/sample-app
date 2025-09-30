use std::collections::HashMap;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use askama::Template;
use axum::extract::Query;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::routing::{get, post};
use axum::Router;
use axum::{extract::State, Form};
use http::HeaderMap;
use http::HeaderValue;
use http::StatusCode;
use serde::Deserialize;
use sqlx::query;
use tower_sessions::Session;

use crate::middleware::is_htmx;
use crate::models::state::WebState;
use crate::templates::base_ctx;
use crate::templates::BaseCtx;
use crate::templates::LoginTmpl;

pub const SESSION_USER_ID: &str = "uid";

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

async fn verify_password(hash: &str, password: &str) -> bool {
    PasswordHash::new(hash)
        .ok()
        .map(|parsed| {
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed)
                .is_ok()
        })
        .unwrap_or(false)
}

pub async fn login_get(Query(q): Query<HashMap<String, String>>, session: Session) -> Html<String> {
    let ctx = base_ctx(&session).await;

    let html = LoginTmpl {
        ctx,
        error: q.contains_key("error"),
    }
    .render()
    .unwrap();
    Html(html)
}

pub async fn login_post(
    State(state): State<WebState>,
    session: Session,
    headers: HeaderMap,
    Form(form): Form<LoginForm>,
) -> axum::response::Response {
    let user = query!(
        r#"SELECT id, email, password_hash, created_at FROM users WHERE email = ?"#,
        form.email
    )
    .fetch_optional(&state.db)
    .await
    .unwrap();

    let password_match =
        matches!(&user, Some(u) if verify_password(&u.password_hash, &form.password).await);

    if password_match {
        if let Some(u) = user {
            session.insert(SESSION_USER_ID, u.id).await.unwrap();
        }

        if is_htmx(&headers) {
            let mut hm = HeaderMap::new();
            hm.insert("HX-Redirect", HeaderValue::from_static("/samples"));
            hm.insert("HX-Replace-Url", HeaderValue::from_static("/samples"));
            return (StatusCode::NO_CONTENT, hm).into_response();
        } else {
            return Redirect::to("/samples").into_response();
        }
    }

    // invalid login
    if is_htmx(&headers) {
        let html = LoginTmpl {
            error: true,
            ctx: BaseCtx {
                is_authenticated: false,
            },
        }
        .render()
        .unwrap();
        (StatusCode::UNAUTHORIZED, Html(html)).into_response()
    } else {
        Redirect::to("/login?error=invalid").into_response()
    }
}

pub async fn logout(session: Session, headers: HeaderMap) -> impl IntoResponse {
    let _ = session.remove::<String>(SESSION_USER_ID).await;
    if is_htmx(&headers) {
        let mut hm = HeaderMap::new();
        hm.insert("HX-Redirect", HeaderValue::from_static("/login"));
        hm.insert("HX-Replace-Url", HeaderValue::from_static("/login"));

        (StatusCode::NO_CONTENT, hm).into_response()
    } else {
        axum::response::Redirect::to("/login").into_response()
    }
}

pub async fn auth_check(session: Session) -> StatusCode {
    match session.get::<i64>(SESSION_USER_ID).await.ok().flatten() {
        Some(_) => StatusCode::NO_CONTENT,
        None => StatusCode::UNAUTHORIZED,
    }
}

pub fn router() -> Router<WebState> {
    Router::new()
        .route("/login", get(login_get).post(login_post))
        .route("/logout", post(logout))
        .route("/auth/check", get(auth_check))
}
