use axum::{extract::{Path, State}, response::IntoResponse, routing::get, Router};
use reqwest::StatusCode;
use crate::app_state::AppStateType;

use super::{static_files, RouterType};

#[inline]
pub(super) fn initialize() -> RouterType {
    Router::new()
        .route("/home", get(home))
        .route("/favicon.ico", get(favicon))
}

async fn home(state: State<AppStateType>) -> Result<impl IntoResponse, StatusCode> {
    static_files::get_static_file(state, Path("index.html".to_string())).await
}

async fn favicon(state: State<AppStateType>) -> Result<impl IntoResponse, StatusCode> {
    static_files::get_static_file(state, Path("favicon.ico".to_string())).await
}
