use std::ops::Deref;

use axum::{
    extract::{FromRequestParts, Query, rejection::QueryRejection},
    http::request::Parts,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::app::error::AppError;

/// Extractor that deserializes the query string into `T` and then runs
/// `validator` validation on it. Rejections are converted into our `AppError`
/// response shape (the same JSON envelope as the rest of the API).
pub struct ValidQuery<T>(pub T);

impl<T> Deref for ValidQuery<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S, T> FromRequestParts<S> for ValidQuery<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Parse the query string into `T` (missing/wrong-typed fields land here).
        let Query(value) = Query::<T>::from_request_parts(parts, state)
            .await
            .map_err(query_rejection_to_response)?;

        // Run the `validator`-derived rules (ranges, etc.).
        value
            .validate()
            .map_err(|err| AppError::InvalidInput(err.to_string()).into_response())?;

        Ok(ValidQuery(value))
    }
}

fn query_rejection_to_response(rejection: QueryRejection) -> Response {
    // e.g. "Failed to deserialize query string: missing field `limit`"
    AppError::BadRequest(rejection.body_text()).into_response()
}
