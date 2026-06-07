use crate::handlers::misc;

pub fn create_router() -> axum::Router {
    axum::Router::new().merge(misc::router())
}
