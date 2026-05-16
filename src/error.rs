// Error Handling - Clean Architecture Layer
// Custom error types with global error handling

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use thiserror::Error;

// ============================================================================
// CUSTOM ERROR TYPE
// ============================================================================

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Unauthorized access")]
    Unauthorized,

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("JWT error: {0}")]
    Jwt(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Image processing error: {0}")]
    ImageProcessing(String),

    #[error("File error: {0}")]
    File(String),

    #[error("Cluster error: {0}")]
    Cluster(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

// ============================================================================
// ERROR RESPONSE STRUCTURE
// ============================================================================

#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

// ============================================================================
// IMPLEMENT AXUM INTO RESPONSE
// ============================================================================

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match &self {
            AppError::Database(_) => {
                tracing::error!("Database error: {}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "database_error", self.to_string())
            }
            AppError::Auth(msg) => {
                tracing::warn!("Authentication error: {}", msg);
                (StatusCode::UNAUTHORIZED, "authentication_error", msg.clone())
            }
            AppError::Unauthorized => {
                tracing::warn!("Unauthorized access attempt");
                (StatusCode::UNAUTHORIZED, "unauthorized", "Authentication required".to_string())
            }
            AppError::Forbidden(msg) => {
                tracing::warn!("Forbidden access: {}", msg);
                (StatusCode::FORBIDDEN, "forbidden", msg.clone())
            }
            AppError::NotFound(msg) => {
                tracing::info!("Not found: {}", msg);
                (StatusCode::NOT_FOUND, "not_found", msg.clone())
            }
            AppError::Validation(msg) => {
                tracing::warn!("Validation error: {}", msg);
                (StatusCode::BAD_REQUEST, "validation_error", msg.clone())
            }
            AppError::Conflict(msg) => {
                tracing::warn!("Conflict: {}", msg);
                (StatusCode::CONFLICT, "conflict", msg.clone())
            }
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", msg.clone())
            }
            AppError::Jwt(msg) => {
                tracing::warn!("JWT error: {}", msg);
                (StatusCode::UNAUTHORIZED, "jwt_error", msg.clone())
            }
            AppError::Io(err) => {
                tracing::error!("IO error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "io_error", err.to_string())
            }
            AppError::ImageProcessing(msg) => {
                tracing::error!("Image processing error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "image_processing_error", msg.clone())
            }
            AppError::File(msg) => {
                tracing::error!("File error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "file_error", msg.clone())
            }
            AppError::Cluster(msg) => {
                tracing::error!("Cluster error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "cluster_error", msg.clone())
            }
            AppError::Serialization(err) => {
                tracing::error!("Serialization error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "serialization_error", err.to_string())
            }
        };

        let error_response = ErrorResponse {
            error: error_type.to_string(),
            message,
            status: status.as_u16(),
            details: None,
        };

        (status, Json(error_response)).into_response()
    }
}

// ============================================================================
// RESULT TYPE ALIAS
// ============================================================================

pub type Result<T> = std::result::Result<T, AppError>;

// ============================================================================
// ERROR CONSTRUCTORS
// ============================================================================

impl AppError {
    pub fn auth(msg: impl Into<String>) -> Self {
        AppError::Auth(msg.into())
    }

    pub fn forbidden(msg: impl Into<String>) -> Self {
        AppError::Forbidden(msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        AppError::NotFound(msg.into())
    }

    pub fn validation(msg: impl Into<String>) -> Self {
        AppError::Validation(msg.into())
    }

    pub fn conflict(msg: impl Into<String>) -> Self {
        AppError::Conflict(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        AppError::Internal(msg.into())
    }

    pub fn jwt(msg: impl Into<String>) -> Self {
        AppError::Jwt(msg.into())
    }

    pub fn image_processing(msg: impl Into<String>) -> Self {
        AppError::ImageProcessing(msg.into())
    }

    pub fn file(msg: impl Into<String>) -> Self {
        AppError::File(msg.into())
    }

    pub fn cluster(msg: impl Into<String>) -> Self {
        AppError::Cluster(msg.into())
    }
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

pub fn validate_email(email: &str) -> Result<()> {
    if email.is_empty() {
        return Err(AppError::validation("Email is required"));
    }

    // Basic email validation
    if !email.contains('@') || !email.contains('.') {
        return Err(AppError::validation("Invalid email format"));
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return Err(AppError::validation("Invalid email format"));
    }

    Ok(())
}

pub fn validate_password(password: &str) -> Result<()> {
    if password.is_empty() {
        return Err(AppError::validation("Password is required"));
    }

    if password.len() < 6 {
        return Err(AppError::validation("Password must be at least 6 characters"));
    }

    Ok(())
}

pub fn validate_required<T: AsRef<str>>(field_name: &str, value: Option<T>) -> Result<String> {
    match value {
        Some(v) if !v.as_ref().is_empty() => Ok(v.as_ref().to_string()),
        _ => Err(AppError::validation(format!("{} is required", field_name))),
    }
}

pub fn validate_pagination(page: Option<i64>, limit: Option<i64>) -> Result<(i64, i64)> {
    let page = page.unwrap_or(1).max(1);
    let limit = limit.unwrap_or(20).min(100).max(1);

    if page < 1 {
        return Err(AppError::validation("Page must be greater than 0"));
    }

    if limit < 1 || limit > 100 {
        return Err(AppError::validation("Limit must be between 1 and 100"));
    }

    Ok((page, limit))
}

// ============================================================================
// ERROR RESPONSE WITH DETAILS
// ============================================================================

impl AppError {
    pub fn with_details(self, details: serde_json::Value) -> ErrorResponse {
        let (status, error_type, message) = match &self {
            AppError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database_error",
                self.to_string(),
            ),
            AppError::Auth(msg) => (StatusCode::UNAUTHORIZED, "authentication_error", msg.clone()),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "unauthorized",
                "Authentication required".to_string(),
            ),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, "forbidden", msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg.clone()),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, "validation_error", msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "conflict", msg.clone()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", msg.clone()),
            AppError::Jwt(msg) => (StatusCode::UNAUTHORIZED, "jwt_error", msg.clone()),
            AppError::Io(err) => (StatusCode::INTERNAL_SERVER_ERROR, "io_error", err.to_string()),
            AppError::ImageProcessing(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "image_processing_error",
                msg.clone(),
            ),
            AppError::File(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "file_error", msg.clone()),
            AppError::Cluster(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "cluster_error", msg.clone()),
            AppError::Serialization(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "serialization_error",
                err.to_string(),
            ),
        };

        ErrorResponse {
            error: error_type.to_string(),
            message,
            status: status.as_u16(),
            details: Some(details),
        }
    }
}
