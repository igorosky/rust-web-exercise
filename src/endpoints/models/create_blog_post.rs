use crate::services::file_handler_service::FileHandle;

#[derive(Debug)]
pub(crate) struct CreateBlogPost {
    pub user_name: String,
    pub content: String,
    pub user_avatar_url: Option<String>,
    pub post_image: Option<FileHandle>,
}
