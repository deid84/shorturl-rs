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

// Change it with the URL where you'll host your project
const BASE_URL: &str = "localhost:8086/";

// Struct to manage frontend form data
#[derive(Deserialize)]
pub struct FormData {
    url: String,
}

pub async fn get_url(
    State(state): State<Arc<AppState>>,
    Path(url): Path<String>,
) -> Response<String> {
    // Attempt to resolve the short URL to its corresponding long URL
    match resolve_short_url(&state.db, url).await {
        Ok(result) => Response::builder()
            // If resolution is successful, construct a response indicating a permanent redirect (status code 301)
            // Set the 'Location' header to the resolved long URL and provide a body with a redirection message
            .status(StatusCode::MOVED_PERMANENTLY)
            .header("Location", result)
            .body("Redirecting...".to_string())
            .unwrap(),
        Err(_) => Response::builder()
            // If an error occurs during resolution, construct a response with a '404 Not Found' status code
            // Include the error message in the response body to indicate the failure
            .status(StatusCode::NOT_FOUND)
            .body("Sorry your short URL does not exist!".to_string())
            .unwrap(),
    }
}

pub async fn post_url(
    State(state): State<Arc<AppState>>,
    Form(form_data): Form<FormData>,
) -> Response<String> {
    // Check if the provided URL is valid
    if Url::parse(&form_data.url).is_ok() {
        // Generate url random part
        match shorten_url(&state.db, form_data.url).await {
            Ok(result) => Response::builder()
                // If shortening is successful, construct a response with status code '200 OK'
                // Include the shortened URL in the response body prefixed with the base URL
                .status(StatusCode::OK)
                .body(format!("{}{}", BASE_URL, result))
                .unwrap(),
            Err(err) => Response::builder()
                // If an error occurs during shortening, construct a response with status code '500 Internal Server Error'
                // Include the error message in the response body to indicate the failure
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(err.to_string())
                .unwrap(),
        }
    } else {
        Response::builder()
            // If the provided URL is not valid, construct a response with status code '500 Internal Server Error'
            // Include a message in the response body indicating that the URL is not valid
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Not a URL!".to_string())
            .unwrap()
    }
}
