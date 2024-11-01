
use std::sync::Arc;

use axum::{extract::{Multipart, State}, http::StatusCode, routing::post, Router};
use super::{models::create_blog_post::CreateBlogPost, AppState, RouterType};

#[inline]
pub(super) fn initialize() -> RouterType {
    Router::new()
        .route("/add", post(add_post))
}

async fn add_post(State(app_state): State<Arc<AppState>>, mut req: Multipart) -> Result<StatusCode, StatusCode> {
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
        app_state.blog_post_service.add_post(CreateBlogPost{user_name, content, user_avatar_url, post_image}).await
            .map_err(|err| {
                tracing::error!("Error adding post: {:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        return Ok(StatusCode::CREATED);
    }
    Err(StatusCode::BAD_REQUEST)
}
