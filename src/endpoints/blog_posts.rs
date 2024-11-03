
use std::sync::Arc;

use axum::{extract::{DefaultBodyLimit, Multipart, Query, State}, http::StatusCode, response::{IntoResponse, Redirect}, routing::{get, post}, Json, Router};
use super::{models::get_posts_response::GetPostsResponse, AppState, RouterType};

#[inline]
pub(super) fn initialize() -> RouterType {
    Router::new()
        .route("/add", post(add_post))
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024))
        .route("/get", get(get_posts))
}

async fn add_post(State(app_state): State<Arc<AppState>>, mut req: Multipart) -> Result<impl IntoResponse, StatusCode> {
    let mut user_name = None;
    let mut content = None;
    let mut user_avatar_url = None;
    let mut post_image = None;
    while let Ok(Some(field)) = req.next_field().await {
        match field.name() {
            Some("user_name") => user_name = Some(field.text().await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?),
            Some("content") => content = Some(field.text().await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?),
            Some("user_avatar_url") => user_avatar_url = Some(field.text().await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?),
            Some("post_image") => {
                if let Some("") = field.file_name() {
                    continue;
                }
                post_image = match app_state.file_handler_service
                    .save_file(field).await {
                    Ok(v) => Some(v),
                    Err(err) => {
                        tracing::error!("Error saving image: {:?}", err);
                        return Err(StatusCode::INTERNAL_SERVER_ERROR)
                    },
                };
            },
            _ => (),
        }
    }
    if let (Some(user_name), Some(content)) = (user_name, content) {
        app_state.blog_post_service.add_post(user_name, content, user_avatar_url, post_image).await
            .map_err(|err| {
                tracing::error!("Error adding post: {:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        
        return Ok(Redirect::to("/home").into_response());
    }
    Err(StatusCode::BAD_REQUEST)
}

#[derive(Debug, Clone, serde::Deserialize)]
struct GetPostsQuery {
    offset: Option<i64>,
    limit: Option<i64>,
}

async fn get_posts(State(app_state): State<Arc<AppState>>, Query(pagination): Query<GetPostsQuery>) -> Result<Json<GetPostsResponse>, StatusCode> {
    let posts = app_state.blog_post_service.get_posts(pagination.limit, pagination.offset).await
        .map_err(|err| {
            tracing::error!("Error getting posts: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(posts))
}
