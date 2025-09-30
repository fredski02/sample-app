use crate::web::auth::SESSION_USER_ID;
use axum::response::IntoResponse;
use axum::{
    extract::FromRequestParts,
    response::{Redirect, Response},
};
use base64::{engine::general_purpose, Engine as _};
use http::HeaderMap;
use http::{header::CONTENT_TYPE, request::Parts, HeaderValue, StatusCode};
use tower_sessions::cookie::time::{self, Duration};
use tower_sessions::cookie::Key;
use tower_sessions::service::SignedCookie;
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};

// ------ Auth related middleware
#[derive(Debug, Clone)]
pub struct AuthedUser(pub i64);

impl<S> FromRequestParts<S> for AuthedUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|_| unauthorized_redirect(parts))?;

        if let Ok(Some(uid)) = session.get::<i64>(SESSION_USER_ID).await {
            Ok(AuthedUser(uid))
        } else {
            Err(unauthorized_redirect(parts))
        }
    }
}

fn unauthorized_redirect(parts: &Parts) -> Response {
    if is_htmx(&parts.headers) {
        let mut resp = Response::new(axum::body::Body::empty());
        *resp.status_mut() = StatusCode::UNAUTHORIZED;
        resp.headers_mut()
            .insert("HX-Redirect", HeaderValue::from_static("/login"));
        return resp;
    }
    if is_json(&parts.headers) {
        let mut resp = Response::new(axum::body::Body::empty());
        *resp.status_mut() = StatusCode::UNAUTHORIZED;
        return resp;
    }
    Redirect::to("/login").into_response()
}

// --------------- Session and session related
pub fn setup_sessions() -> SessionManagerLayer<MemoryStore, SignedCookie> {
    let secure_session = std::env::var("SECURE_SESSION")
        .unwrap_or("true".to_string())
        .parse::<bool>()
        .unwrap_or(true);

    let signing_key = load_session_key();

    let store = MemoryStore::default();

    SessionManagerLayer::new(store)
        .with_name("sample_session")
        .with_secure(secure_session)
        .with_same_site(tower_sessions::cookie::SameSite::Lax)
        .with_http_only(true)
        .with_expiry(Expiry::AtDateTime(
            time::OffsetDateTime::now_utc() + Duration::days(30),
        ))
        .with_signed(signing_key)
}

fn load_session_key() -> Key {
    if let Ok(key_b64) = std::env::var("SESSION_SIGNING_KEY_BASE64") {
        let bytes = general_purpose::STANDARD
            .decode(key_b64)
            .expect("invalid base64 key");
        Key::from(&bytes)
    } else {
        Key::generate()
    }
}

// --------------- Header parsers
pub fn is_htmx(headers: &HeaderMap) -> bool {
    headers
        .get("HX-Request")
        .map(|v| v == "true")
        .unwrap_or(false)
}

fn is_json(headers: &HeaderMap) -> bool {
    headers
        .get(CONTENT_TYPE)
        .unwrap_or(&HeaderValue::from_str("").unwrap())
        == "application/json"
}
