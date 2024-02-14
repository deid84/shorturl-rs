use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{Response, StatusCode},
    Form,
};
use serde::Deserialize;
use url::Url;

use super::{
    service::{resolve_short_url, shorten_url},
    AppState,
};

#[derive(Deserialize)]
pub struct FormData {
    url: String,
}

pub async fn get_url(
    State(state): State<Arc<AppState>>,
    Path(url): Path<String>,
) -> Response<String> {
    match resolve_short_url(&state.db, url).await {
        Ok(result) => Response::builder()
            .status(StatusCode::OK)
            .body(result)
            .unwrap(),
        Err(err) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(err.to_string())
            .unwrap(),
    }
}

pub async fn post_url(
    State(state): State<Arc<AppState>>,
    Form(form_data): Form<FormData>,
) -> Response<String> {
    if let Ok(_) = Url::parse(&form_data.url) {
        // Generate url random part
        match shorten_url(&state.db, form_data.url).await {
            Ok(result) => Response::builder()
                .status(StatusCode::OK)
                .body(result)
                .unwrap(),
            Err(err) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(err.to_string())
                .unwrap(),
        }
    } else {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(format!("Not a URL!"))
            .unwrap()
    }
}
