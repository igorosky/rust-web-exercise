use std::sync::Arc;
use crate::{db::DatabasePool, env_variables, services::{blog_post_service::BlogPostService, file_handler_service::FileHandlerService, static_files_service::StaticFilesService}};

pub(crate) type AppStateType = Arc<AppState>;

#[derive(Debug, thiserror::Error)]
pub(crate) enum AppStateInitializationError {
    #[error("{0}")]
    GettingEnvVarError(#[from] env_variables::GettingEnvVarError),
    #[error("Invalid path")]
    InvalidPathError,
    #[error("Invalid number")]
    NotValidNumber,
}

pub(crate) struct AppState {
    pub blog_post_service: BlogPostService,
    pub file_handler_service: FileHandlerService,
    pub static_files_service: StaticFilesService,
}

impl AppState {
    #[inline]
    pub(crate) fn new(
        blog_post_service: BlogPostService,
        file_handler_service: FileHandlerService,
        static_files_service: StaticFilesService,
    ) -> Self {
        Self {
            blog_post_service,
            file_handler_service,
            static_files_service,
        }
    }

    #[inline]
    pub(crate) async fn initialize(connection_pool: DatabasePool) -> Result<Arc<Self>, AppStateInitializationError> {
        use env_variables::get_env_var as var;
        let ans = Arc::new(Self::new(
            BlogPostService::new(connection_pool.clone()),
            FileHandlerService::new(
                connection_pool,
                var(env_variables::UPLOAD_DIRECTORY)?.as_str(),
                var(env_variables::UPLOAD_BUFFER_SIZE)?
                    .parse().map_err(|_| AppStateInitializationError::NotValidNumber)?,
            ).ok_or(AppStateInitializationError::InvalidPathError)?,
            StaticFilesService::new(
                var(env_variables::STATIC_FILES_DIRECTORY)?.as_str()
            ).ok_or(AppStateInitializationError::InvalidPathError)?,
        ));
        let ptr = Arc::downgrade(&ans);
        ans.blog_post_service.set_app_state(ptr).await;
        Ok(ans)
    }
}
