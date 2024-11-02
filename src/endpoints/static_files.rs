use std::sync::Arc;

use axum::{body::Body, extract::{Path, State}, response::IntoResponse, routing::get, Router};
use reqwest::StatusCode;
use super::{AppState, RouterType};

#[inline]
pub(super) fn initialize() -> RouterType {
    Router::new()
        .route("/*path", get(get_static_file))
}

pub(super) async fn get_static_file(State(app_state): State<Arc<AppState>>, Path(path): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    Ok(Body::from_stream(app_state.static_files_service.get_static_file(&path).await
        .map_err(|err| match err.kind() {
            tokio::io::ErrorKind::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?
    ))
}