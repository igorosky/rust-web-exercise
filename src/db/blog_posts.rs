use time::PrimitiveDateTime;

use super::DatabasePool;

#[derive(Debug, Clone, sqlx::FromRow)]
pub(crate) struct BlogPost {
    pub id: i64,
    pub user_name: String,
    pub content: String,
    pub user_avatar: Option<i64>,
    pub post_image: Option<i64>,
    pub publication_date: PrimitiveDateTime,
}

#[inline]
pub(crate) async fn insert_post(
    pool: &DatabasePool,
    user_name: &str,
    content: &str,
    user_avatar: Option<i64>,
    post_image: Option<i64>
) -> Result<BlogPost, sqlx::Error> {
    sqlx::query_as::<_, BlogPost>(
        "INSERT INTO BlogPosts (user_name, content, user_avatar, post_image) VALUES (?, ?, ?, ?) RETURNING *",
    )
        .bind(user_name)
        .bind(content)
        .bind(user_avatar)
        .bind(post_image)
        .fetch_one(pool)
        .await
}

// #[inline]
// pub(crate) async fn get_user_by_id(pool: &DatabasePool, id: i64) -> Result<Option<User>, sqlx::Error> {
//     sqlx::query_as::<_, User>(
//         "SELECT * FROM users WHERE id = ?"
//     )
//         .bind(id)
//         .fetch_optional(pool)
//         .await
// }
