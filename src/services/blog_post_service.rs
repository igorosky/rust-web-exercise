use std::sync::Weak;
use tokio::sync::Mutex;
use crate::{app_state::AppState, db::{blog_posts, DatabasePool}, endpoints::models::get_posts_response::GetPostsResponse};
use super::file_handler_service::FileHandle;

pub(crate) struct BlogPostService {
    connection_pool: DatabasePool,
    app_state: Mutex<Weak<AppState>>,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum AddingBlogPostError {
    #[error("Failed to fetch user avatar: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Failed to access database: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("User avatar is not an png image")]
    UserAvatarIsNotAnPNGImage,
    #[error("Failed to fetch user avatar")]
    FailedToFetchUserAvatar,
    #[error("Tokio IO error: {0}")]
    TokioIoError(#[from] tokio::io::Error),
    #[error("User avatar is too big")]
    UserAvatarIsTooBig,
}

impl BlogPostService {
    #[inline]
    pub(crate) fn new(connection_pool: DatabasePool) -> Self {
        Self {
            connection_pool,
            app_state: Mutex::new(Weak::new()),
        }
    }

    pub(crate) async fn add_post(
        &self, 
        user_name: String,
        content: String,
        mut user_avatar_url: Option<String>,
        mut post_image: Option<FileHandle>,
    ) -> Result<(), AddingBlogPostError> {
        let mut user_avatar = None;
        user_avatar_url = user_avatar_url.take()
            .map(|v| v.trim().to_string())
            .and_then(|v| if v.is_empty() { None } else { Some(v) });
        if let Some(user_avatar_url) = user_avatar_url.as_mut() {
            let response = reqwest::get(user_avatar_url.as_str()).await?;
            if !response.status().is_success() {
                return Err(AddingBlogPostError::FailedToFetchUserAvatar);
            }

            // Some images does not have Content-Type header or even though they are PNG images, they are not marked as such
            // let is_image= response.headers()
            //     .get("Content-Type")
            //     .and_then(|v| v.to_str().ok())
            //     .map(|v| v == "image/png")
            //     .unwrap_or(false);
            // if !is_image {
            //     return Err(AddingBlogPostError::UserAvatarIsNotAnPNGImage);
            // }
            
            let user_avatar_tmp = self.app_state.lock().await.upgrade()
                .expect("Service do not have a valid reference to app state")
                .file_handler_service
                .save_file(response.bytes_stream()).await.map_err(|err| {
                    use super::file_handler_service::FileHandlerServiceError;
                    match err {
                        FileHandlerServiceError::TokioIoError(err) => err.into(),
                        FileHandlerServiceError::SqlxError(err) => err.into(),
                        FileHandlerServiceError::FileIsNotAnPNGImage => AddingBlogPostError::UserAvatarIsNotAnPNGImage,
                        FileHandlerServiceError::FileIsTooBig => AddingBlogPostError::UserAvatarIsTooBig,
                    }
                })?;
            *user_avatar_url = user_avatar_tmp.get_name()
                .and_then(|v| v.to_str())
                .map(|v| v.to_string())
                .ok_or(AddingBlogPostError::TokioIoError(
                    tokio::io::Error::new(tokio::io::ErrorKind::InvalidData, "Failed to parse file name")))?;
            user_avatar = Some(user_avatar_tmp);
        }

        for image in [post_image.as_mut(), user_avatar.as_mut()].into_iter().flatten() {
            image.save().await.map_err(|err| {
                use super::file_handler_service::FileHandleSaveError;
                use tokio::io::{Error, ErrorKind};
                match err {
                    FileHandleSaveError::SqlxError(err) => err.into(),
                    FileHandleSaveError::FileNameParsingError => AddingBlogPostError::TokioIoError(
                        Error::new(ErrorKind::InvalidData, "Failed to parse file name")),
            }})?;
        }
        blog_posts::insert_post(
            &self.connection_pool,
            &user_name,
            &content,
            user_avatar.and_then(|v| v.get_id()),
            post_image.and_then(|v| v.get_id()),
        ).await?;
        Ok(())
    }

    #[inline]
    pub(crate) async fn set_app_state(&self, app_state: Weak<AppState>) {
        *self.app_state.lock().await = app_state;
    }
    
    pub(crate) async fn get_posts(&self, limit: Option<i64>, offset: Option<i64>) -> Result<GetPostsResponse, sqlx::Error> {
    let limit = limit.map(|v| v.clamp(1, 100)).unwrap_or(10);
        let offset = offset.map(|v| v.max(0)).unwrap_or(0);
        Ok(
            GetPostsResponse {
                limit,
                offset,
                total: blog_posts::get_total_amount_of_posts(&self.connection_pool).await?,
                posts: blog_posts::get_newest_posts(
                    &self.connection_pool,
                    limit,
                    offset,
                ).await?,
            }
        )
    }
    
    #[inline]
    pub(crate) async fn get_posts_all(&self) -> Result<Vec<blog_posts::Post>, sqlx::Error> {
        blog_posts::get_all_newest_posts(&self.connection_pool).await
    }
}
