use crate::db::blog_posts::Post;

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct GetPostsResponse {
    pub limit: i64,
    pub offset: i64,
    pub total: i64,
    pub posts: Vec<Post>,
}