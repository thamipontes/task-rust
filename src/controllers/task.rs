use axum::{extract::Extension, Json};
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::AppError, models::{self, task::Task}};

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

    let result = sqlx::query("insert into task (name, id) values ($1, $2)")
        .bind(&task_request.name)
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
        Ok(Json(json!({"message": "Task registered sucessfully"})))
    }
    
    
    }


    pub async fn find_all(Extension(pool): Extension<PgPool>) -> Result<Json<Vec<Task>>, AppError> {

        let tasks = sqlx::query_as::<_, models::task::Task>(
            "SELECT name, id, created_at, updated_at FROM task",
        )
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            dbg!(err);
            AppError::InternalServerError
        })?;
    
        if tasks.is_empty() {
            return Err(AppError::TaskDoesNotExist);
        } else {
            return Ok(Json(tasks));
        }
        
        }