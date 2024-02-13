use std::{os::linux::raw::stat, sync::Arc};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Form, Router,
};
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;
use sqlx::{PgPool, Pool, Postgres};

use self::service::shorten_url;

mod model;
mod service;

struct AppState {
    db: Pool<Postgres>,
}

#[derive(Deserialize)]
struct FormData {
    url: String,
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
        .route("/", get(home).post(set_url))
        .route("/:url", get(get_url))
        .with_state(app_state)
}

async fn home() -> Result<String, StatusCode> {
    Err(StatusCode::NOT_FOUND)
}

async fn get_url(
    State(state): State<Arc<AppState>>,
    Path(url): Path<String>,
) -> Result<String, StatusCode> {
    Err(StatusCode::NOT_FOUND)
}

async fn set_url(
    State(state): State<Arc<AppState>>,
    Form(form_data): Form<FormData>,
) -> Result<String, StatusCode> {
    // Generate url random part
    shorten_url(&state.db, form_data.url).await.unwrap();
    
    Ok(format!("OK"))
}
