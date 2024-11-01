use std::sync::Arc;

use axum::{extract::{Path, State}, response::IntoResponse, routing::get, Router};
use reqwest::StatusCode;
use super::{AppState, RouterType};

#[inline]
pub(super) fn initialize() -> RouterType {
    Router::new()
        .route("/:uuid", get(get_image))
}

async fn get_image(State(app_state): State<Arc<AppState>>, Path(uuid): Path<String>) -> impl IntoResponse {
    let mut response = app_state.file_handler_service.get_file(&uuid).await
        .map(|v| v.into_response())
        .unwrap_or_else(|_| StatusCode::NOT_FOUND.into_response());
    response.headers_mut().insert("Content-Type", "image/png".parse().unwrap());
    response
}
