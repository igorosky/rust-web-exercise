use std::path::PathBuf;

use tokio::fs::File;
use tokio_util::io::ReaderStream;


pub(crate) struct StaticFilesService {
    static_files_directory: PathBuf,
}

impl StaticFilesService {
    pub(crate) fn new(static_files_directory: &str) -> Option<Self> {
        let static_files_directory = PathBuf::from(static_files_directory);
        match static_files_directory.is_dir() {
            true => Some(Self { static_files_directory }),
            false => None,
        }
    }

    pub(crate) async fn get_static_file(&self, file_name: &str) -> Result<ReaderStream<File>, tokio::io::Error> {
        let mut path = self.static_files_directory.clone();
        path.push(file_name);
        if !path.is_file() {
            return Err(tokio::io::Error::new(tokio::io::ErrorKind::NotFound, "File not found"));
        }
        let file = File::open(path).await?;
        Ok(ReaderStream::new(file))
    }
}
