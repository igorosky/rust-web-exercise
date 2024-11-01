use std::sync::Arc;

use axum::{body::Body, extract::State, response::IntoResponse, routing::get, Router};
use reqwest::StatusCode;
use super::{AppState, RouterType};

#[inline]
pub(super) fn initialize() -> RouterType {
    Router::new()
        .route("/home", get(home))
}

async fn home(State(app_state): State<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    Ok(Body::from_stream(
        app_state.static_files_service.get_static_file("index.html").await
            .map_err(|err| match err.kind() {
                tokio::io::ErrorKind::NotFound => axum::http::StatusCode::NOT_FOUND,
                _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            })?
    ))
}