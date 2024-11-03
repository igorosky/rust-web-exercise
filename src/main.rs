mod endpoints;
mod app_state;
mod db;
mod services;
mod env_variables;

use endpoints::start_server;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use app_state::AppState;
use crate::db::initialize_db;

#[inline]
fn tracing_subscriber_init() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::DEBUG.into())
                .from_env()
                .expect("Invalid logger configuration")
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    env_variables::debug_mode_initialization();
    tracing_subscriber_init();
    tracing::info!("Starting the application");

    // Database initialization
    tracing::info!("Initializing a connection pool to the database");
    let database_url = match env_variables::get_env_var(env_variables::DATABASE_URL) {
        Ok(database_url) => database_url,
        Err(err) => {
            tracing::error!("{}", err);
            return;
        }
    };
    let connection_pool = match initialize_db(database_url.as_str()).await {
        Ok(connection_pool) => connection_pool,
        Err(err) => {
            tracing::error!("Error while initializing the database: {}", err);
            return;
        }
    };
    tracing::info!("Database connection pool has been initialized successfully");


    tracing::info!("Starting the server");
    let app_state = match AppState::initialize(
        connection_pool.clone(),
    ).await {
        Ok(app_state) => app_state,
        Err(err) => {
            tracing::error!("{}", err);
            return;
        },
    };
    if let Err(err) = start_server(app_state).await {
        tracing::error!("Error while running server: {}", err);
        return;
    }
    tracing::info!("Server has been shut down successfully");
    tracing::info!("Closing the database connection pool");
    connection_pool.close().await;
    tracing::info!("Database connection pool has been closed successfully");
    tracing::info!("Application has been shut down successfully");
}
