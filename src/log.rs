use axum::extract::MatchedPath;
use http::Request;
use std::time::Duration;
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{DefaultMakeSpan, DefaultOnFailure, OnRequest, OnResponse, TraceLayer},
};
use tracing::{info, Level, Span};

#[derive(Clone)]
pub struct LogOnRequest;

impl<B> OnRequest<B> for LogOnRequest {
    fn on_request(&mut self, req: &Request<B>, _span: &Span) {
        let route = req
            .extensions()
            .get::<MatchedPath>()
            .map(MatchedPath::as_str)
            .unwrap_or("<unmatched>");
        info!(
            target: "http",
            method = %req.method(),
            path = %req.uri().path(),
            route = %route,
            "request"
        );
    }
}

#[derive(Clone)]
pub struct LogOnResponse;

impl<B> OnResponse<B> for LogOnResponse {
    fn on_response(self, res: &axum::response::Response<B>, latency: Duration, _span: &Span) {
        info!(
            target: "http",
            status = %res.status().as_u16(),
            latency_ms = %latency.as_millis(),
            "response"
        );
    }
}

pub fn setup_logging() -> TraceLayer<
    SharedClassifier<ServerErrorsAsFailures>,
    DefaultMakeSpan,
    LogOnRequest,
    LogOnResponse,
> {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "sample-app=debug,http=info,tower_http=info".into()),
        )
        .compact()
        .init();

    TraceLayer::new_for_http()
        .on_request(LogOnRequest)
        .on_response(LogOnResponse)
        .on_failure(DefaultOnFailure::new().level(Level::WARN))
}
