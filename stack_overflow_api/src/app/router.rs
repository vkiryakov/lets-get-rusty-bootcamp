use std::time::Duration;

use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    sensitive_headers::SetSensitiveHeadersLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::app::request_context;
use crate::app::state::AppState;
use crate::handlers::{misc, question};

/// Maximum allowed request body size (2 MiB).
const REQUEST_BODY_LIMIT: usize = 2 * 1024 * 1024;
/// Maximum time a request is allowed to run before being aborted.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

pub fn create_router(state: AppState) -> axum::Router {
    // Layers are applied top-to-bottom: the first layer is the outermost
    // (runs first on the request, last on the response).
    let middleware = ServiceBuilder::new()
        // Generate a unique request id and propagate it to the response.
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id())
        // Capture request id / method / path into a task-local so error
        // responses (and anything else in the request task) can read them.
        .layer(axum::middleware::from_fn(
            request_context::capture_request_context,
        ))
        // Catch panics in handlers and turn them into 500 responses.
        .layer(CatchPanicLayer::new())
        // Emit structured tracing spans/events for every request and response,
        // including the request id (set by SetRequestIdLayer above) in the span.
        .layer(TraceLayer::new_for_http().make_span_with(
            |request: &axum::http::Request<_>| {
                let request_id = request
                    .headers()
                    .get("x-request-id")
                    .and_then(|value| value.to_str().ok())
                    .unwrap_or("unknown");

                tracing::info_span!(
                    "request",
                    method = %request.method(),
                    uri = %request.uri(),
                    request_id = %request_id,
                )
            },
        ))
        // Don't log sensitive headers.
        .layer(SetSensitiveHeadersLayer::new([
            axum::http::header::AUTHORIZATION,
            axum::http::header::COOKIE,
        ]))
        // Permissive CORS; tighten for production as needed.
        .layer(CorsLayer::permissive())
        // Compress responses (gzip, br, zstd, deflate) based on Accept-Encoding.
        .layer(CompressionLayer::new())
        // Reject request bodies larger than the limit (used by extractors).
        .layer(DefaultBodyLimit::max(REQUEST_BODY_LIMIT))
        // Abort requests that run longer than the timeout.
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            REQUEST_TIMEOUT,
        ));

    // Build the router by merging the individual handler routers and applying
    axum::Router::new()
        .merge(misc::router())
        .merge(question::router())
        .layer(middleware)
        .with_state(state)
}
