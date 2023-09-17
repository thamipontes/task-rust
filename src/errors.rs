use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    MissingName,
    InternalServerError,
    TaskAlreadyExists
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "an internal server error occured",),
            Self::MissingName => (StatusCode::BAD_REQUEST, "Missing Name"),
            Self::TaskAlreadyExists => (StatusCode::BAD_REQUEST, "Task aleready exists")
        };

        (status, Json(json!({"error" : err_msg}))).into_response()
    }
}