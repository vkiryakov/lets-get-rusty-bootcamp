use std::ops::Deref;

use axum::{
    extract::{FromRequestParts, Path, rejection::PathRejection},
    http::request::Parts,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;

use crate::app::error::AppError;

/// Extractor that deserializes path parameters into `T`. Rejections (e.g. an
/// invalid UUID) are converted into our `AppError` response shape instead of
/// axum's default plain-text error.
pub struct ValidPath<T>(pub T);

impl<T> Deref for ValidPath<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S, T> FromRequestParts<S> for ValidPath<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Send,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(value) = Path::<T>::from_request_parts(parts, state)
            .await
            .map_err(path_rejection_to_response)?;

        Ok(ValidPath(value))
    }
}

fn path_rejection_to_response(rejection: PathRejection) -> Response {
    // e.g. "Invalid URL: Cannot parse `id` with value `123x`: UUID parsing failed"
    AppError::BadRequest(rejection.body_text()).into_response()
}
