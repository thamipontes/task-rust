use axum::{extract::Extension, Json};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{errors::AppError, models};

pub async fn register(Extension(pool): Extension<PgPool>, Json(task_request): Json<models::task::NewTask>) -> Result<Json<Value>, AppError> {
    if task_request.name.is_empty() {
        return Err(AppError::MissingName);
    }

    let task = sqlx::query_as::<_, models::task::Task>(
        "SELECT name FROM task where name = $1",
    )
    .bind(&task_request.name)
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        dbg!(err);
        AppError::InternalServerError
    })?;

    if let Some(_) = task {
        return Err(AppError::TaskAlreadyExists);
    }

    let teste = &task_request.name;

    let result = sqlx::query("insert into task (name) values ($1)")
        .bind(teste)
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
        Ok(Json(json!({"message": "Task registered sucessfully"})))
    }
    
    
    }