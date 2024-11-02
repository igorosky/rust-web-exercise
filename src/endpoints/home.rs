use std::sync::Arc;

use axum::{extract::{Path, State}, response::IntoResponse, routing::get, Router};
use reqwest::StatusCode;
use super::{static_files, AppState, RouterType};

#[inline]
pub(super) fn initialize() -> RouterType {
    Router::new()
        .route("/home", get(home))
}

async fn home(state: State<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    static_files::get_static_file(state, Path("index.html".to_string())).await
}