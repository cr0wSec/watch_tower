use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tracing::error;
#[derive(Error, Debug)]
pub enum AppError {
    #[error("template rendering failed: {0}")]
    Template(#[from] askama::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Template(_) => {
                error!(error = %self, "request failed");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal error occurred. Please try again later.".to_string(),
                )
            }
        };
        (status, message).into_response()
    }
}
