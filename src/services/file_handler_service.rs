use std::{error, path::{Path, PathBuf}};
use axum::body::Bytes;
use futures::{pin_mut, Stream, TryStreamExt};
use sha2::Digest;
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
use tokio_util::io::StreamReader;

use crate::db::{image::{get_image_by_hash, insert_image}, DatabasePool};

const BUFFER_SIZE: usize = 1024;

#[derive(Debug, Clone)]
pub(crate) struct FileHandlerService {
    connection_pool: DatabasePool,
    folder_path: PathBuf,
}

#[derive(Debug)]
pub(crate) struct FileHandle {
    id: Option<i64>,
    path: PathBuf,
    is_saved: bool,
    connection_pool: DatabasePool,
    image_hash: Vec<u8>,
}

impl FileHandle {
    #[inline]
    pub(crate) async fn save(&mut self) -> Result<(), sqlx::error::Error> {
        if !self.is_saved {
            self.is_saved = true;
            self.id = Some(insert_image(
                &self.connection_pool,
                &self.image_hash,
                self.get_name().unwrap().to_str().unwrap().to_string()
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

impl FileHandlerService {
    pub(crate) fn new(connection_pool: DatabasePool, folder_path: &str) -> Option<Self> {
        let folder_path = Path::new(folder_path).to_owned();
        match folder_path.is_dir() {
            true => Some(Self { connection_pool, folder_path }),
            false => None,
        }
    }

    pub(crate) async fn save_file(
        &self,
        content: impl Stream<Item = Result<Bytes, impl Into<Box<dyn error::Error + Send + Sync>>>>
    ) -> Result<FileHandle, Box<dyn std::error::Error>> {
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
        let reader = StreamReader::new(content.map_err(|err| tokio::io::Error::new(tokio::io::ErrorKind::Other, err)));
        let mut hasher = sha2::Sha256::new();
        let mut buffer = [0; BUFFER_SIZE];
        pin_mut!(reader);
        let mut read_bytes_count = reader.read(&mut buffer).await?;
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
    
    pub(crate) async fn get_file(&self, filename: &str) -> Result<Bytes, tokio::io::Error> {
        let mut file_path = self.folder_path.clone();
        file_path.push(filename);
        let mut buffer = Vec::new();
        File::open(file_path).await?.read_to_end(&mut buffer).await?;
        Ok(buffer.into())
    }
}
