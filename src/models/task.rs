use serde::{Deserialize, Serialize};


#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Task {
    pub id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct NewTask {
    pub name: String,
}