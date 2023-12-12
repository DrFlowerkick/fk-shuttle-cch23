//!app_error.rs

use anyhow::anyhow;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt::Display;

// Make our own error that wraps StatusCode and `anyhow::Error`.
pub struct AppError {
    status_code: StatusCode,
    err: anyhow::Error,
}

impl AppError {
    pub fn bad_request(err: impl Display) -> Self {
        Self {
            status_code: StatusCode::BAD_REQUEST,
            err: anyhow!("{}", err),
        }
    }
    pub fn to_bad_request(err: impl Into<anyhow::Error>) -> Self {
        Self {
            status_code: StatusCode::BAD_REQUEST,
            err: err.into(),
        }
    }
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.status_code,
            format!("Something went wrong: {}", self.err),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            err: err.into(),
        }
    }
}

// type alias for Result
pub type AppResult<T> = std::result::Result<T, AppError>;
