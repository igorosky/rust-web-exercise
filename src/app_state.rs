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
    pub(crate) async fn initialize(connection_pool: DatabasePool, uploads_directory: &str) -> Arc<Self> {
        let ans = Arc::new(Self::new(
            BlogPostService::new(connection_pool.clone()),
            FileHandlerService::new(connection_pool, uploads_directory).unwrap(),
            StaticFilesService::new("static_files").unwrap(),
        ));
        let ptr = Arc::downgrade(&ans);
        ans.blog_post_service.set_app_state(ptr.clone()).await;
        ans
    }
}
