use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    MissingName,
    InternalServerError,
    TaskAlreadyExists,
    MissingCredential,
    WrongCredential,
    UserDoesNotExist,
    TokenCreation,
    InvalidToken,
    UserAlreadyExist,
    TaskDoesNotExist,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "an internal server error occured",),
            Self::MissingName => (StatusCode::BAD_REQUEST, "Missing Name"),
            Self::TaskAlreadyExists => (StatusCode::BAD_REQUEST, "Task aleready exists"),
            Self::MissingCredential => (StatusCode::BAD_REQUEST, "Missing Credentials"),
            Self::WrongCredential => (StatusCode::UNAUTHORIZED, "Wrong Credentials"),
            Self::UserDoesNotExist => (StatusCode::UNAUTHORIZED, "User does not exist"),
            Self::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create token"),
            Self::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid Token"),
            Self::UserAlreadyExist => (StatusCode::BAD_REQUEST, "User already exist"),
            Self::TaskDoesNotExist => (StatusCode::BAD_REQUEST, "Task doesn't exists")
        };

        (status, Json(json!({"error" : err_msg}))).into_response()
    }
}