use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub struct ApiError {
    code: StatusCode,
    message: String,
}

impl ApiError {
    #[allow(dead_code)]
    pub fn new(code: StatusCode, message: String) -> Self {
        Self { code, message }
    }

    pub fn new_internal(message: String) -> ApiError {
        ApiError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message,
        }
    }

    pub fn new_not_found(message: String) -> ApiError {
        ApiError {
            code: StatusCode::NOT_FOUND,
            message,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let payload = json!({
            "message": self.message
        });
        (self.code, Json(payload)).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        ApiError::new_internal(error.to_string())
    }
}
