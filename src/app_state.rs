use std::sync::Arc;

use crate::{db::DatabasePool, services::{blog_post_service::BlogPostService, file_handler_service::FileHandlerService}};

pub(crate) struct AppState {
    pub blog_post_service: BlogPostService,
    pub file_handler_service: FileHandlerService,
}

impl AppState {
    #[inline]
    pub(crate) fn new(
        blog_post_service: BlogPostService,
        file_handler_service: FileHandlerService,
    ) -> Self {
        Self {
            blog_post_service,
            file_handler_service,
        }
    }

    #[inline]
    pub(crate) async fn new_with_defaults(connection_pool: DatabasePool, uploads_directory: &str) -> Arc<Self> {
        let ans = Arc::new(Self::new(
            BlogPostService::new(connection_pool.clone()),
            FileHandlerService::new(connection_pool, uploads_directory).unwrap(),
        ));
        let ptr = Arc::downgrade(&ans);
        ans.blog_post_service.set_app_state(ptr.clone()).await;
        ans
    }
}
