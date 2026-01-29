use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use anyhow::Ok;
use axum::body::Bytes as AxumBytes;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::{common::store_type::StoreType, store::object_store::ObjectStore};

pub struct StandaloneStore {}
impl StandaloneStore {
    pub fn new() -> Self {
        Self {}
    }

    fn gen_next_filename(path: &Path) -> anyhow::Result<String> {
        let next_file_number = path
            .read_dir()?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_file())
            .filter_map(|path| {
                path.file_stem()
                    .and_then(|stem| stem.to_str())
                    .and_then(|stem_str| stem_str.parse::<u32>().ok())
            })
            .max()
            .map_or(0, |n| n + 1);

        Ok(format!("{}.{}", next_file_number, Self::STORE_EXTENSION))
    }
}

impl ObjectStore for StandaloneStore {
    fn path() -> PathBuf {
        std::env::current_dir()
            .unwrap()
            .join("store")
            .join("standalone")
    }

    fn save(&self, bytes: &AxumBytes) -> anyhow::Result<StoreType> {
        let path = Self::path();
        if !path.exists() {
            fs::create_dir_all(&path).context("not able to create parent path")?;
        }

        let next_filename = Self::gen_next_filename(&path)?;
        let next_file_path = path.join(next_filename);

        fs::write(&next_file_path, &bytes).context("failed to write binaries to the file")?;

        Ok(StoreType::Standalone {
            file_path: next_file_path,
        })
    }

    async fn open(
        &self,
        file_name: &str,
    ) -> anyhow::Result<futures::stream::BoxStream<'static, Result<AxumBytes, futures::io::Error>>>
    {
        let path = Self::path().join(file_name);
        let file = File::open(path).await?;
        let stream = ReaderStream::new(file);
        // Ok(stream.boxed())
        Ok(Box::pin(stream))
    }
}

#[cfg(test)]
pub mod test {
    use futures::StreamExt;
    use rstest::rstest;
    use tokio::fs;

    use crate::{
        common::store_type::StoreType,
        store::{object_store::ObjectStore, standalone_store::StandaloneStore},
    };

    #[rstest]
    #[tokio::test]
    async fn test_save_and_open() {
        let standalone_store = StandaloneStore::new();
        let bytes = fs::read(
            std::env::current_dir()
                .unwrap()
                .join("test")
                .join("test-image.jpeg"),
        )
        .await
        .unwrap();

        let store_type = standalone_store
            .save(&axum::body::Bytes::from(bytes.clone()))
            .unwrap();

        if let StoreType::Standalone { file_path } = store_type {
            assert!(file_path.file_name().is_some());
            let file_name = file_path.file_name().unwrap();
            let mut stream = standalone_store
                .open(file_name.to_str().unwrap())
                .await
                .unwrap();

            let mut binary = Vec::new();
            while let Some(chunk) = stream.next().await {
                binary.extend_from_slice(&chunk.unwrap());
            }

            assert_eq!(
                binary, bytes,
                "binary read from the standalone store is different",
            );
            assert!(
                bytes.len() > 1_000_000,
                "test_image should be larger than 1MB"
            );
        } else {
            panic!("standalone store returns no store type")
        }
    }
}
