use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

/// Per-request context captured from the incoming request.
///
/// Stored in a task-local so it can be read from anywhere within the same
/// request task — including `IntoResponse` impls, which otherwise have no
/// access to the request.
#[derive(Clone, Debug)]
pub struct RequestContext {
    pub request_id: String,
    pub method: String,
    pub path: String,
}

tokio::task_local! {
    static REQUEST_CONTEXT: RequestContext;
}

impl RequestContext {
    /// Read the current request context, if we're inside the request scope.
    /// Returns `None` when called outside of a request (e.g. startup).
    pub fn current() -> Option<RequestContext> {
        REQUEST_CONTEXT.try_with(|ctx| ctx.clone()).ok()
    }
}

/// Middleware that captures the request id / method / path and makes them
/// available via [`RequestContext::current`] for the duration of the request.
///
/// Must run *after* the request id has been set on the request (i.e. inside
/// `SetRequestIdLayer`).
pub async fn capture_request_context(request: Request, next: Next) -> Response {
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let ctx = RequestContext {
        request_id,
        method: request.method().to_string(),
        path: request.uri().path().to_string(),
    };

    REQUEST_CONTEXT.scope(ctx, next.run(request)).await
}
