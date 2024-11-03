use std::path::PathBuf;

use tokio::fs::File;
use tokio_util::io::ReaderStream;

use super::file_handler_service::get_file_from_directory;


pub(crate) struct StaticFilesService {
    static_files_directory: PathBuf,
}

impl StaticFilesService {
    pub(crate) fn new(static_files_directory: &str) -> Option<Self> {
        let static_files_directory = PathBuf::from(static_files_directory).canonicalize().ok()?;
        match static_files_directory.is_dir() {
            true => Some(Self { static_files_directory }),
            false => None,
        }
    }

    #[inline]
    pub(crate) async fn get_static_file(&self, file_name: &str) -> Result<ReaderStream<File>, tokio::io::Error> {
        get_file_from_directory(self.static_files_directory.clone(), file_name).await
    }
}
