use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

/// Custom application-specific error enum.
/// This allows us to map various internal errors to standardized HTTP responses.
#[derive(Debug)]
pub enum AppError {
    InternalServerError(String), // For unhandled server errors, with a message
    // NotFound(String),            // For resources not found (e.g., user, property)
    Conflict(String),            // For resource conflicts (e.g., username already taken)
    Unauthorized(String),        // For authentication failures
    // BadRequest(String),          // For invalid request data
    // Forbidden(String),           // For authorization failures (e.g., not allowed to access resource)
}

// --- Implement `IntoResponse` for `AppError` ---
// This tells Axum how to convert our `AppError` enum into an HTTP response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            // AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            // AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            // AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
        };

        // Construct a JSON response body
        let body = Json(json!({
            "error": error_message,
        }));

        // Return the HTTP response
        (status, body).into_response()
    }
}

// --- Implement `From` traits for common error conversions ---
// These conversions allow us to use the `?` operator on other error types
// and have them automatically converted into an `AppError`.

/// Converts `sqlx::Error` into `AppError::InternalServerError`.
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::InternalServerError(format!("Database error: {}", err))
    }
}

/// Converts `argon2::Error` into `AppError::InternalServerError`.
impl From<argon2::Error> for AppError {
    fn from(err: argon2::Error) -> Self {
        AppError::InternalServerError(format!("Password hashing error: {}", err))
    }
}

/// Converts `jsonwebtoken::errors::Error` into `AppError::Unauthorized`.
impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::Unauthorized(format!("Authentication token error: {}", err))
    }
}
