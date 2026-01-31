use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("template rendering failed: {0}")]
    Template(#[from] askama::Error),

    #[error("validation error: {0}")]
    Validation(String),
}

impl AppError {
    pub fn validation(errors: validator::ValidationErrors) -> Self {
        let messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |e| {
                    e.message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| format!("Invalid {field}"))
                })
            })
            .collect();

        Self::Validation(messages.join(", "))
    }
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
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        };

        (status, message).into_response()
    }
}
