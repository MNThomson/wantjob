use std::time::Duration;

use anyhow::{Context, Result};
use axum::{
    extract::MatchedPath,
    http::{header, Request, Version},
    response::Response,
    Router,
};
use opentelemetry::trace::SpanKind;
use tokio::net::TcpListener;
use tower_http::{
    catch_panic::CatchPanicLayer, classify::ServerErrorsFailureClass, trace::TraceLayer,
};
use tracing::{debug, info, info_span, Span};

mod components;
mod index;
mod staticfiles;
use crate::db::DbPool;

#[derive(Clone)]
pub struct AppState {
    db: DbPool,
}

pub async fn serve(db: DbPool) -> Result<()> {
    let app = api_router().with_state(AppState { db }).layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                // https://github.com/open-telemetry/semantic-conventions/blob/v1.23.0/docs/http/http-spans.md
                let matched_route = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);
                return info_span!(
                    "http_request",
                    "otel.name" = format!("{} {}", request.method(), matched_route.unwrap_or("UnknownRoute")),
                    "otel.kind" = format!("{:?}", SpanKind::Server),
                    "otel.status_code" = tracing::field::Empty,
                    "http.request.method" = ?request.method(),
                    "http.route" = matched_route,
                    "url.path" = request.uri().path(),
                    "url.query" = request.uri().query(),
                    "user_agent.original" = request.headers().get(header::USER_AGENT).and_then(|val| val.to_str().ok()),
                    "http.flavor" = match request.version() {
                        Version::HTTP_09 => "0.9",
                        Version::HTTP_10 => "1.0",
                        Version::HTTP_11 => "1.1",
                        Version::HTTP_2 => "2.0",
                        Version::HTTP_3 => "3.0",
                        _ => "Unknown",
                    },
                    "http.request.content_length" = request.headers().get(header::CONTENT_LENGTH).and_then(|val| val.to_str().ok()),
                    "http.response.status_code" = tracing::field::Empty,
                    "error.type" = tracing::field::Empty,
                );
            })
            .on_request(|_request: &Request<_>, _span: &Span| {})
            .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                _span.record("http.response.status_code", _response.status().as_u16() as i64);
                debug!("Request served");
            })
            .on_failure(
                |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                    _span.record("error.type", _error.to_string());
                    _span.record("otel.status_code", "ERROR");
                    debug!("Request errored");
                }
            )
    );

    let listener = TcpListener::bind("127.0.0.1:4321").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .context("error running HTTP server")
}

fn api_router() -> Router<AppState> {
    return Router::new()
        .merge(index::router())
        .merge(staticfiles::router())
        .layer(CatchPanicLayer::new());
}
