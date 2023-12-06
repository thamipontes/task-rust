use axum::{extract::Extension, Json};
use jsonwebtoken::{encode, Header};
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{self, auth::Claims},
    utils::get_timestamp_8_hours_from_now,
    KEYS,
};

pub async fn login(Json(credencials): Json<models::auth::User>,
Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, AppError> {
    if credencials.email.is_empty() || credencials.password.is_empty() {
        return Err(AppError::MissingCredential);
    }

    let user = sqlx::query_as::<_, models::auth::User>(
        "SELECT email, password FROM user where email = $1",
    )
    .bind(&credencials.email)
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        dbg!(err);
        AppError::InternalServerError
    })?;

    if let Some(user) = user {
        if user.password != credencials.password{
            Err(AppError::WrongCredential)
        } else {
            let claims = Claims {
                email: credencials.email.to_owned(),
                exp: get_timestamp_8_hours_from_now(),
            };
            let token = encode(&Header::default(), &claims, &KEYS.encoding)
                .map_err(|_| AppError::TokenCreation)?;
            Ok(Json(json!({"access_token": token, "type": "Bearer"})))
        }
    } else {
        Err(AppError::UserDoesNotExist)
    }
}


pub async fn create_user(Extension(pool): Extension<PgPool>, Json(user_request): Json<models::auth::NewUser>) -> Result<Json<Value>, AppError> {
    if (user_request.email.is_empty() || user_request.password.is_empty()) {
        return Err(AppError::MissingCredential);
    }

    let user = sqlx::query_as::<_, models::auth::User>(
        "SELECT (email, password) FROM user where email = $1",
    )
    .bind(&user_request.email)
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        dbg!(err);
        AppError::InternalServerError
    })?;

    if let Some(_) = user {
        return Err(AppError::UserAlreadyExist);
    }

    let result = sqlx::query("insert into user (name, email, id) values ($1, $2, $3)")
        .bind(&user_request.email)
        .bind(&user_request.password)
        .bind(Uuid::new_v4())
        .execute(&pool)
        .await
        .map_err(|err| {
            dbg!(err);
            AppError::InternalServerError
        })?;

    let affected_rows = result.rows_affected();

    if affected_rows < 1 {
        Err(AppError::InternalServerError)
    }else {
        Ok(Json(json!({"message": "User registered sucessfully"})))
    }
    }


    pub async fn user_profile(claims: Claims) -> Result<axum::Json<serde_json::Value>, AppError> {
        Ok(axum::Json(serde_json::json!({"email": claims.email})))
    }
