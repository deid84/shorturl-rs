use std::sync::Arc;

use anyhow::Result;
use axum::{routing::get, Router};
use sqlx::{PgPool, Pool, Postgres};

mod handler;
mod model;
mod service;

/// This struct represents the application state, holding a database connection pool
struct AppState {
    db: Pool<Postgres>,
}

/// This function serves as the entry point for running the Axum web server.
/// It takes a PostgreSQL connection pool (`PgPool`) as input, sets up the application state,
/// creates the API routes using the provided application state, binds the server to a specific port,
/// and starts serving incoming connections.
pub async fn serve(db: PgPool) -> Result<()> {
    // Set up the application state with the provided database connection pool
    let app_state = AppState { db };
    // Create the API routes using the application state
    let app = api_router(Arc::new(app_state));
    let port = 8086;
    // Bind the server to the specified address and port
    let address = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    // Start serving incoming connections
    axum::serve(address, app.into_make_service()).await?;
    Ok(())
}

/// This function defines the API routes for the application.
/// It takes the application state as input and sets up the routes for handling different HTTP methods and endpoints.
fn api_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handler::app).post(handler::post_url))
        .route("/:url", get(handler::get_url))
        .with_state(app_state)
}
