mod endpoints;
mod app_state;
mod db;
mod services;

use endpoints::start_server;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use app_state::AppState;

use crate::db::initialize_db;

#[inline]
#[cfg(debug_assertions)]
fn debug_mode_initialization() {
    dotenvy::dotenv().ok();
}

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
    debug_mode_initialization();
    tracing_subscriber_init();
    tracing::info!("Starting the application");

    // Database initialization
    tracing::info!("Initializing a connection pool to the database");
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(database_url) => database_url,
        Err(_) => {
            tracing::error!("DATABASE_URL environment variable is not set");
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
        Err(std::env::VarError::NotPresent) => {
            tracing::error!("Error while initializing the application state: environment variable is not set");
            return;
        }
        Err(std::env::VarError::NotUnicode(_)) => {
            tracing::error!("Error while initializing the application state: environment variable is not a valid Unicode string");
            return;
        }
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
