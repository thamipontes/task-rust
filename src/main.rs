use axum::{
    extract::{Extension},routing::{get, post}, Router,
};

use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::fs;
use anyhow::Context;

mod errors;
mod models;
mod controllers;


#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let env = fs::read_to_string(".env").unwrap();
    let (key, database_url) = env.split_once('=').unwrap();

    assert_eq!(key, "DATABASE_URL");

    let pool = PgPoolOptions::new()
    .max_connections(100)
    .connect(&database_url)
    .await
    .context("could not connect to database_url")?;

    let migrations_dir = "./migrations";

    let entries = fs::read_dir(migrations_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let migration_sql = tokio::fs::read_to_string(&path).await?;
            sqlx::query(&migration_sql).execute(&pool).await?;
        }
    }


    let app = Router::new()
        .route("/hello", get(root))
        .route("/register", post(controllers::task::register))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

        Ok(())

}

async fn root() -> &'static str {
    "Hello, World!"
}

