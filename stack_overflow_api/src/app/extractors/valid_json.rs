use std::ops::Deref;

use axum::{
    Json,
    extract::{FromRequest, Request, rejection::JsonRejection},
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::app::error::AppError;

/// Extractor that deserializes the request body as JSON into `T` and then runs
/// `validator` validation on it. Rejections are converted into our `AppError`
/// response shape.
pub struct ValidJson<T>(pub T);

impl<T> Deref for ValidJson<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S, T> FromRequest<S> for ValidJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // 1. Parse the body as JSON into `T` (handles content-type, syntax, shape).
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(json_rejection_to_response)?;

        // 2. Run the `validator`-derived rules (length, ranges, etc.).
        value
            .validate()
            .map_err(|err| AppError::InvalidInput(err.to_string()).into_response())?;

        Ok(ValidJson(value))
    }
}

fn json_rejection_to_response(rejection: JsonRejection) -> Response {
    let message = rejection.body_text();

    let error = match rejection {
        // Valid JSON, but it doesn't match the expected structure -> 422.
        JsonRejection::JsonDataError(_) => AppError::InvalidInput(message),
        // Malformed JSON / missing content-type / failed to read body -> 400.
        _ => AppError::BadRequest(message),
    };

    error.into_response()
}
