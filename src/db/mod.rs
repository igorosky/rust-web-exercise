#[allow(dead_code)]
pub(crate) mod blog_posts;
#[allow(dead_code)]
pub(crate) mod image;

pub(crate) type Database = sqlx::Sqlite;
pub(crate) type DatabasePool = sqlx::Pool<Database>;

pub(crate) async fn initialize_db(database_url: &str) -> Result<DatabasePool, sqlx::error::Error> {
    tracing::debug!("Connecting to the database at {}", database_url);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(database_url)
        .await?;
    tracing::debug!("Connected to the database");
    tracing::debug!("Migrating the database");
    sqlx::migrate!("./migrations").run(&pool).await?;
    tracing::debug!("Database migrated successfully");
    Ok(pool)
}
