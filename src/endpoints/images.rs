use std::sync::Arc;

use axum::{body::Body, extract::{Path, State}, response::IntoResponse, routing::get, Router};
use reqwest::StatusCode;
use super::{AppState, RouterType};

#[inline]
pub(super) fn initialize() -> RouterType {
    Router::new()
        .route("/:uuid", get(get_image))
}

async fn get_image(
    State(app_state): State<Arc<AppState>>, Path(uuid): Path<String>
) -> Result<impl IntoResponse, StatusCode> {
    let mut response = Body::from_stream(app_state.file_handler_service.get_file(&uuid).await
    .map_err(|err| match err.kind() {
        tokio::io::ErrorKind::NotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?).into_response();
    response.headers_mut()
        .insert(
            "Content-Type",
            "image/png".parse().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        );
    Ok(response)
}
