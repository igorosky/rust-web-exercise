use std::sync::Weak;

use tokio::sync::Mutex;

use crate::{app_state::AppState, db::{blog_posts::{insert_post, BlogPost}, DatabasePool}, endpoints::models::create_blog_post::CreateBlogPost};

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

    pub(crate) async fn add_post(&self, mut blog_post: CreateBlogPost) -> Result<BlogPost, Box<dyn std::error::Error>> {
        let mut user_avatar = None;
        blog_post.user_avatar_url = blog_post.user_avatar_url.take()
            .map(|v| v.trim().to_string())
            .and_then(|v| if v.is_empty() { None } else { Some(v) });
        if let Some(user_avatar_url) = blog_post.user_avatar_url.as_mut() {
            let response = reqwest::get(user_avatar_url.as_str()).await.unwrap();
            if !response.status().is_success() {
                return Err(String::from("Failed to fetch user avatar").into());
            }
            let uuid = uuid::Uuid::new_v4().to_string();
            user_avatar = Some(self.app_state.lock().await.upgrade().unwrap()
                .file_handler_service
                .save_file(response.bytes_stream()).await?);
            *user_avatar_url = uuid;
        }
        if let Some(image) = blog_post.post_image.as_mut() {
            image.save().await?;
        }
        if let Some(image) = user_avatar.as_mut() {
            image.save().await?;
        }
        Ok(insert_post(
            &self.connection_pool,
            &blog_post.user_name,
            &blog_post.content,
            user_avatar.and_then(|v| v.get_id()),
            blog_post.post_image.and_then(|v| v.get_id()),
        ).await?)
    }

    #[inline]
    pub(crate) async fn set_app_state(&self, app_state: Weak<AppState>) {
        *self.app_state.lock().await = app_state;
    }
}
