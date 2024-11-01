use super::DatabasePool;

#[derive(Debug, Clone, sqlx::FromRow)]
pub(crate) struct Image {
    id: i64,
    image_filename: String,
}

impl Image {
    #[inline]
    pub(crate) fn get_id(&self) -> i64 {
        self.id
    }

    pub(crate) fn get_filename(&self) -> &str {
        self.image_filename.as_str()
    }
}

#[inline]
pub(crate) async fn insert_image(pool: &DatabasePool, image_hash: &[u8], image_filename: String) -> Result<Image, sqlx::Error> {
    sqlx::query_as::<_, Image>(
        "INSERT INTO Images (image_hash, image_filename) VALUES (?, ?) RETURNING *"
    )
        .bind(image_hash)
        .bind(image_filename)
        .fetch_one(pool)
        .await
}

#[inline]
pub(crate) async fn get_image_by_hash(pool: &DatabasePool, image_hash: &[u8]) -> Result<Option<Image>, sqlx::Error> {
    sqlx::query_as::<_, Image>(
        "SELECT id, image_filename FROM Images WHERE image_hash = ?"
    )
        .bind(image_hash)
        .fetch_optional(pool)
        .await
}
