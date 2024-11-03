use axum::{body::Body, extract::{Path, State}, response::IntoResponse, routing::get, Router};
use reqwest::StatusCode;
use super::{AppStateType, RouterType};

#[inline]
pub(super) fn initialize() -> RouterType {
    Router::new()
        .route("/:uuid", get(get_image))
}

async fn get_image(
    State(app_state): State<AppStateType>, Path(uuid): Path<String>
) -> Result<impl IntoResponse, StatusCode> {
    let mut response = Body::from_stream(app_state.file_handler_service.get_file(&uuid).await
    .map_err(|err| {
        use crate::services::file_handler_service::GetFileFromDirectoryError;
        match err {
            GetFileFromDirectoryError::FileNotFound => StatusCode::NOT_FOUND,
            GetFileFromDirectoryError::PathNotInAllowedDirectory => StatusCode::FORBIDDEN,
            GetFileFromDirectoryError::TokioIoError(_) => {
                tracing::error!("Error getting image: {:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            },
        }
    })?).into_response();
    response.headers_mut()
        .insert(
            "Content-Type",
            "image/png".parse().map_err(|_| {
                tracing::error!("Error creating response header");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
        );
    Ok(response)
}
