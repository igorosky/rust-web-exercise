use std::{error, path::{Path, PathBuf}};
use axum::body::Bytes;
use futures::{pin_mut, Stream, TryStreamExt};
use sha2::Digest;
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
use tokio_util::io::{ReaderStream, StreamReader};
use crate::db::{image::{get_image_by_hash, insert_image}, DatabasePool};

const PNG_HEADER: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

#[derive(Debug, thiserror::Error)]
pub(crate) enum GetFileFromDirectoryError {
    #[error("File not found")]
    FileNotFound,
    #[error("Path is not in allowed directory")]
    PathNotInAllowedDirectory,
    #[error("Tokio io error: {0}")]
    TokioIoError(#[from] tokio::io::Error),
}

// It is expected that folder_path is canonicalized
pub(super) async fn get_file_from_directory(
    folder_path: PathBuf,
    filename: &str
) -> Result<ReaderStream<File>, GetFileFromDirectoryError> {
    let mut path = folder_path.clone();
    path.push(filename);

    // HTTP protocol should ensure that ".." will not take place however better safe than sorry
    path = path.canonicalize()?;
    if !path.starts_with(folder_path) {
        return Err(GetFileFromDirectoryError::PathNotInAllowedDirectory);
    }
    
    if !path.is_file() {
        return Err(GetFileFromDirectoryError::FileNotFound);
    }
    Ok(ReaderStream::new(File::open(path).await?))
}

#[derive(Debug)]
pub(crate) struct FileHandle {
    id: Option<i64>,
    path: PathBuf,
    is_saved: bool,
    connection_pool: DatabasePool,
    image_hash: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum FileHandleSaveError {
    #[error("Failed to access database: {0}")]
    SqlxError(#[from] sqlx::error::Error),
    #[error("Failed to parse file name")]
    FileNameParsingError,
}

impl FileHandle {
    #[inline]
    pub(crate) async fn save(&mut self) -> Result<(), FileHandleSaveError> {
        if !self.is_saved {
            self.is_saved = true;
            self.id = Some(insert_image(
                &self.connection_pool,
                &self.image_hash,
                self.get_name()
                    .and_then(|v| v.to_str())
                    .map(|v| v.to_string())
                    .ok_or(FileHandleSaveError::FileNameParsingError)?
            ).await?.get_id());
        }
        Ok(())
    }
    
    #[inline]
    pub(crate) fn get_name(&self) -> Option<&std::ffi::OsStr> {
        self.path.file_name()
    }

    pub(crate) fn get_id(&self) -> Option<i64> {
        self.id
    }
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        if !self.is_saved {
            let path = self.path.clone();
            tokio::spawn(async {
                tokio::fs::remove_file(path).await
            });
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FileHandlerService {
    connection_pool: DatabasePool,
    folder_path: PathBuf,
    buffer_size: usize,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum FileHandlerServiceError {
    #[error("Tokio Io Error: {0}")]
    TokioIoError(#[from] tokio::io::Error),
    #[error("File is not an PNG image")]
    FileIsNotAnPNGImage,
    #[error("Failed to access database: {0}")]
    SqlxError(#[from] sqlx::error::Error),
}

impl FileHandlerService {
    pub(crate) fn new(connection_pool: DatabasePool, folder_path: &str, buffer_size: usize) -> Option<Self> {
        let folder_path = Path::new(folder_path).to_owned();
        match folder_path.is_dir() {
            true => Some(Self { connection_pool, folder_path: folder_path.canonicalize().ok()?, buffer_size }),
            false => None,
        }
    }

    pub(crate) async fn save_file(
        &self,
        content: impl Stream<Item = Result<Bytes, impl Into<Box<dyn error::Error + Send + Sync>>>>
    ) -> Result<FileHandle, FileHandlerServiceError> {
        let mut file_path = self.folder_path.clone();
        let filename = uuid::Uuid::new_v4().to_string();
        file_path.push(filename);
        let mut file_handle = FileHandle {
            id: None,
            path: file_path.clone(),
            is_saved: false,
            connection_pool: self.connection_pool.clone(),
            image_hash: Vec::new(),
        };
        let mut file = File::create(file_path).await?;
        let reader = StreamReader::new(
            content.map_err(|err| {
                use tokio::io::{Error, ErrorKind};
                Error::new(ErrorKind::Other, err)
            })
        );
        let mut hasher = sha2::Sha256::new();
        let mut buffer = vec![0; self.buffer_size];
        pin_mut!(reader);
        let mut read_bytes_count = reader.read_exact(&mut buffer[..PNG_HEADER.len()]).await?;
        if read_bytes_count < PNG_HEADER.len() || buffer[..8] != PNG_HEADER {
            return Err(FileHandlerServiceError::FileIsNotAnPNGImage);
        }
        while read_bytes_count != 0 {
            hasher.update(&buffer[..read_bytes_count]);
            file.write_all(&buffer[..read_bytes_count]).await?;
            read_bytes_count = reader.read(&mut buffer).await?;
        }
        let image_hash = hasher.finalize().to_vec();
        let existing_image = get_image_by_hash(&self.connection_pool, &image_hash).await?;
        if let Some(existing_image) = existing_image {
            let mut path = self.folder_path.clone();
            path.push(existing_image.get_filename());
            file_handle = FileHandle{
                id: Some(existing_image.get_id()),
                path,
                is_saved: true,
                connection_pool: self.connection_pool.clone(),
                image_hash,
            };
        }
        else {
            file_handle.image_hash = image_hash;
        }
        Ok(file_handle)
    }
    
    #[inline]
    pub(crate) async fn get_file(&self, filename: &str) -> Result<ReaderStream<File>, GetFileFromDirectoryError> {
        get_file_from_directory(self.folder_path.clone(), filename).await
    }
}
