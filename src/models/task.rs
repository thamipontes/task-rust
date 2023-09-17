use serde::{Deserialize, Serialize};


#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Task {
    pub id: i64,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct NewTask {
    pub name: String,
}