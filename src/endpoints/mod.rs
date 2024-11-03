pub(crate) mod models;
mod blog_posts;
mod images;
mod home;
mod static_files;
use axum::Router;
use crate::{app_state::AppStateType, env_variables};

pub(super) type RouterType = Router<AppStateType>;

pub(super) async fn start_server(app_state: AppStateType) -> Result<(), Box<dyn std::error::Error>> {
    let router = Router::new()
        .nest("/post", blog_posts::initialize(
            env_variables::get_env_var(env_variables::MAX_BODY_SIZE)?.parse()?))
        .nest("/image", images::initialize())
        .nest("/file", static_files::initialize())
        .nest("/", home::initialize())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(
        env_variables::get_env_var(env_variables::ADDRESS)?).await?;
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

#[cfg(debug_assertions)]
const SHUTDOWN_CONFIRMATION_MESSAGE: &str = "Are you sure you want to shut down the server? Press Ctrl+C again to confirm";

#[cfg(debug_assertions)]
async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("Failed to listen for the signal");
    tracing::warn!("{}", SHUTDOWN_CONFIRMATION_MESSAGE);
    let mut last_clicked = tokio::time::Instant::now();
    tokio::signal::ctrl_c().await.expect("Failed to listen for the signal");
    let mut second_click = tokio::time::Instant::now();
    while second_click.checked_duration_since(last_clicked).expect("Time went backwards") > tokio::time::Duration::from_secs(5) {
        tracing::warn!("{}", SHUTDOWN_CONFIRMATION_MESSAGE);
        last_clicked = second_click;
        tokio::signal::ctrl_c().await.expect("Failed to listen for the signal");
        second_click = tokio::time::Instant::now();
    }
    tracing::info!("Shutting down the server");
}

#[cfg(not(debug_assertions))]
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
    tracing::info!("Shutting down the server");
}
