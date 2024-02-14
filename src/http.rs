use std::sync::Arc;

use anyhow::Result;
use axum::{http::StatusCode, routing::get, Router};
use sqlx::{PgPool, Pool, Postgres};

mod handler;
mod model;
mod service;

struct AppState {
    db: Pool<Postgres>,
}

pub async fn serve(db: PgPool) -> Result<()> {
    let app_state = AppState { db };
    let app = api_router(Arc::new(app_state));
    let port = 8086;
    let address = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(address, app.into_make_service()).await?;
    Ok(())
}

fn api_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(home).post(handler::post_url))
        .route("/:url", get(handler::get_url))
        .with_state(app_state)
}

async fn home() -> Result<String, StatusCode> {
    Err(StatusCode::NOT_FOUND)
}
