use std::sync::Arc;

use crate::{db::DatabasePool, services::{blog_post_service::BlogPostService, file_handler_service::FileHandlerService, static_files_service}};

use self::static_files_service::StaticFilesService;

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
    pub(crate) async fn initialize(connection_pool: DatabasePool) -> Result<Arc<Self>, std::env::VarError> {
        use std::env::var;
        let ans = Arc::new(Self::new(
            BlogPostService::new(connection_pool.clone()),
            FileHandlerService::new(
                connection_pool,
                var("UPLOAD_DIRECTORY")?.as_str(),
                var("UPLOAD_BUFFER_SIZE")?.parse().unwrap(),
            ).unwrap(),
            StaticFilesService::new(
                var("STATIC_FILES_DIRECTORY")?.as_str()
            ).unwrap(),
        ));
        let ptr = Arc::downgrade(&ans);
        ans.blog_post_service.set_app_state(ptr).await;
        Ok(ans)
    }
}
