use axum::{
    extract::{Extension},routing::{get, post}, Router,
};

use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
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

    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::EnvFilter::new(
    //         std::env::var("tower_http=trace")
    //             .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
    //     ))
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    assert_eq!(key, "DATABASE_URL");

    tracing_subscriber::fmt::init();

    let pool = PgPoolOptions::new()
    .max_connections(100)
    .connect(&database_url)
    .await
    .context("could not connect to database_url")?;

    let app = Router::new()
        .route("/hello", get(root))
        .route("/register", post(controllers::task::register))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

        Ok(())

}

async fn root() -> &'static str {
    "Hello, World!"
}