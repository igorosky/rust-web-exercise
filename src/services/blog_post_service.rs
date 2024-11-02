use std::sync::Weak;
use tokio::sync::Mutex;
use crate::{app_state::AppState, db::{blog_posts::insert_post, DatabasePool}, endpoints::models::get_posts_response::GetPostsResponse};
use super::file_handler_service::FileHandle;

pub(crate) struct BlogPostService {
    connection_pool: DatabasePool,
    app_state: Mutex<Weak<AppState>>,
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
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut user_avatar = None;
        user_avatar_url = user_avatar_url.take()
            .map(|v| v.trim().to_string())
            .and_then(|v| if v.is_empty() { None } else { Some(v) });
        if let Some(user_avatar_url) = user_avatar_url.as_mut() {
            let response = reqwest::get(user_avatar_url.as_str()).await.unwrap();
            if !response.status().is_success() {
                return Err(String::from("Failed to fetch user avatar").into());
            }
            let user_avatar_tmp = self.app_state.lock().await.upgrade().unwrap()
                .file_handler_service
                .save_file(response.bytes_stream()).await?;
            *user_avatar_url = user_avatar_tmp.get_name().unwrap().to_str().unwrap().to_string();
            user_avatar = Some(user_avatar_tmp);
        }
        if let Some(image) = post_image.as_mut() {
            image.save().await?;
        }
        if let Some(image) = user_avatar.as_mut() {
            image.save().await?;
        }
        insert_post(
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
        use crate::db::blog_posts;
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
}
