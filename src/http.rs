use std::sync::Arc;

use anyhow::Result;
use axum::{extract::{Path, State}, http::StatusCode, routing::{get, post}, Router};
use sqlx::{PgPool, Pool, Postgres};

struct AppState {
    db: Pool<Postgres>
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
    .route("/", get(get_root))
    .with_state(app_state)
}

async fn get_root(State(state): State<Arc<AppState>>) -> Result<String, StatusCode> {
    Err(StatusCode::NOT_FOUND)
}