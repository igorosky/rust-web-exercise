
use axum::{extract::{multipart::Field, DefaultBodyLimit, Multipart, Query, State}, http::StatusCode, response::{IntoResponse, Redirect, Response}, routing::{get, post}, Json, Router};
use crate::app_state::AppStateType;
use super::{models::get_posts_response::GetPostsResponse, RouterType};

#[inline]
pub(super) fn initialize(max_body_size: usize) -> RouterType {
    Router::new()
        .route("/add", post(add_post))
        .layer(DefaultBodyLimit::max(max_body_size))
        .route("/get", get(get_posts))
        .route("/get_all", get(get_posts_all))
}

async fn get_field_text(field: Field<'_>) -> Result<Option<String>, StatusCode> {
    let text = field.text().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let text = text.trim().to_string();
    match text.is_empty() {
        true => Ok(None),
        false => Ok(Some(text)),
    }
}

fn create_redirection_with_params(destination: &str, params: &[(&str, &str)]) -> Result<Response, StatusCode> {
    use std::borrow::Cow;
    let mut destination = Cow::Borrowed(destination);
    for (index, (key, value)) in params.iter()
        .map(|(key, value)| (key, urlencoding::encode(value)))
        .enumerate() {
        if index == 0 {
            destination = Cow::Owned(format!("{}?{}={}", destination, key, value));
        } else {
            destination = Cow::Owned(format!("{}&{}={}", destination, key, value));
        }
    }
    Ok(Redirect::to(&destination).into_response())
}

async fn add_post(
    State(app_state): State<AppStateType>,
    mut req: Multipart
) -> Result<Response, StatusCode> {
    let mut user_name = None;
    let mut content = None;
    let mut user_avatar_url = None;
    let mut post_image = None;
    while let Ok(Some(field)) = req.next_field().await {
        match field.name() {
            Some("user_name") => user_name = get_field_text(field).await?,
            Some("content") => content = get_field_text(field).await?,
            Some("user_avatar_url") => user_avatar_url = get_field_text(field).await?,
            Some("post_image") => {
                if let Some("") = field.file_name() {
                    continue;
                }
                post_image = match app_state.file_handler_service
                    .save_file(field).await {
                    Ok(v) => Some(v),
                    Err(err) => {
                        use crate::services::file_handler_service::FileHandlerServiceError;
                        return match err {
                            FileHandlerServiceError::FileIsNotAnPNGImage => 
                                create_redirection_with_params("/home", &[("error", "File is not an PNG image")]),
                            FileHandlerServiceError::SqlxError(_) | FileHandlerServiceError::TokioIoError(_) => {
                                tracing::error!("Error saving image: {:?}", err);
                                create_redirection_with_params("/home", &[("error", "Internal server error")])
                            }
                            FileHandlerServiceError::FileIsTooBig => 
                                create_redirection_with_params("/home", &[("error", "File is too big")])
                        }
                    },
                };
            },
            _ => (),
        }
    }
    match (user_name, content) {
        (Some(user_name), Some(content)) => {
            match app_state.blog_post_service.add_post(user_name, content, user_avatar_url, post_image).await {
                Ok(_) => Ok(Redirect::to("/home").into_response()),
                Err(err) => {
                    tracing::error!("Error adding post: {:?}", err);
                    create_redirection_with_params(
                        "/home",
                        &[("error", "User avatar url does not lead to a PNG")])
                },
            }
        },
        (None, None) => create_redirection_with_params("/home", &[("error", "User name and content cannot be empty (or contain only whit spaces)")]),
        (None, _) => create_redirection_with_params("/home", &[("error", "User name cannot be empty (or contain only whit spaces)")]),
        (_, None) => create_redirection_with_params("/home", &[("error", "Content cannot be empty (or contain only whit spaces)")]),
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct GetPostsQuery {
    offset: Option<i64>,
    limit: Option<i64>,
}

async fn get_posts(State(app_state): State<AppStateType>, Query(pagination): Query<GetPostsQuery>) -> Result<Json<GetPostsResponse>, StatusCode> {
    let posts = app_state.blog_post_service.get_posts(pagination.limit, pagination.offset).await
        .map_err(|err| {
            tracing::error!("Error getting posts: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(posts))
}

async fn get_posts_all(
    State(app_state): State<AppStateType>
) -> Result<Json<Vec<crate::db::blog_posts::Post>>, StatusCode> {
    let posts = app_state.blog_post_service.get_posts_all().await
        .map_err(|err| {
            tracing::error!("Error getting posts: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(posts))
}
