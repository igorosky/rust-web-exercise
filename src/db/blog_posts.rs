use super::DatabasePool;

#[inline]
pub(crate) async fn insert_post(
    pool: &DatabasePool,
    user_name: &str,
    content: &str,
    user_avatar: Option<i64>,
    post_image: Option<i64>
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO BlogPosts (user_name, content, user_avatar, post_image) VALUES (?, ?, ?, ?) RETURNING *",
    )
        .bind(user_name)
        .bind(content)
        .bind(user_avatar)
        .bind(post_image)
        .execute(pool)
        .await?;
    Ok(())
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub(crate) struct Post {
    pub id: i64,
    pub user_name: String,
    pub content: String,
    pub user_avatar: Option<String>,
    pub post_image: Option<String>,
    pub publication_date: String,
}

#[inline]
pub(crate) async fn get_newest_posts(pool: &DatabasePool, limit: i64, offset: i64) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as::<_, Post>(
        "SELECT BlogPosts.id, user_name, content, user_avatar_table.image_filename AS user_avatar, post_image_table.image_filename AS post_image, publication_date
        FROM BlogPosts
        LEFT JOIN Images AS user_avatar_table ON BlogPosts.user_avatar = user_avatar_table.id
        LEFT JOIN Images AS post_image_table ON BlogPosts.post_image = post_image_table.id
        ORDER BY publication_date DESC
        LIMIT ?
        OFFSET ?",
    )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

#[inline]
pub(crate) async fn get_total_amount_of_posts(pool: &DatabasePool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT COUNT(*) FROM BlogPosts")
        .fetch_one(pool)
        .await
}
