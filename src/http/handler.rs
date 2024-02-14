use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Form,
};
use serde::Deserialize;

use super::{service::{resolve_short_url, shorten_url}, AppState};

#[derive(Deserialize)]
pub struct FormData {
    url: String,
}

pub async fn get_url(
    State(state): State<Arc<AppState>>,
    Path(url): Path<String>,
) -> Result<String, StatusCode> {
    let result = resolve_short_url(&state.db, url).await.unwrap();
    Ok(result)
}

pub async fn post_url(
    State(state): State<Arc<AppState>>,
    Form(form_data): Form<FormData>,
) -> Result<String, StatusCode> {
    // Generate url random part
    let result = shorten_url(&state.db, form_data.url).await.unwrap();

    Ok(result)
}
