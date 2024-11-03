use axum::{body::Body, extract::{Path, State}, response::IntoResponse, routing::get, Router};
use reqwest::StatusCode;
use crate::app_state::AppStateType;
use super::RouterType;

#[inline]
pub(super) fn initialize() -> RouterType {
    Router::new()
        .route("/*path", get(get_static_file))
}

pub(super) async fn get_static_file(State(app_state): State<AppStateType>, Path(path): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    use crate::services::file_handler_service::GetFileFromDirectoryError;
    Ok(Body::from_stream(app_state.static_files_service.get_static_file(&path).await
        .map_err(|err| match err {
            GetFileFromDirectoryError::FileNotFound => StatusCode::NOT_FOUND,
            GetFileFromDirectoryError::PathNotInAllowedDirectory => StatusCode::FORBIDDEN,
            GetFileFromDirectoryError::TokioIoError(_) => {
                tracing::error!("Error getting file: {:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            },
        })?
    ))
}