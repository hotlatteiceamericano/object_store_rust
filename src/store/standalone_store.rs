use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use anyhow::Ok;
use axum::body::Bytes as AxumBytes;
use futures::StreamExt;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::{common::store_type::StoreType, store::object_store::ObjectStore};

pub struct StandaloneStore {
    path: PathBuf,
}
impl StandaloneStore {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
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

    pub async fn open(
        path: &PathBuf,
    ) -> anyhow::Result<futures::stream::BoxStream<'static, Result<AxumBytes, futures::io::Error>>>
    {
        let file = File::open(path).await?;
        let stream = ReaderStream::new(file);

        Ok(stream.boxed())
    }
}

impl ObjectStore for StandaloneStore {
    async fn save(&mut self, bytes: &AxumBytes) -> anyhow::Result<StoreType> {
        if !&self.path.exists() {
            fs::create_dir_all(&self.path).context("not able to create parent path")?;
        }

        let next_filename = Self::gen_next_filename(&self.path)?;
        let next_file_path = self.path.join(next_filename);

        fs::write(&next_file_path, &bytes).context("failed to write binaries to the file")?;

        Ok(StoreType::Standalone {
            file_path: next_file_path,
        })
    }
}

#[cfg(test)]
pub mod test {
    use futures::StreamExt;
    use rstest::{fixture, rstest};
    use tokio::fs;

    use crate::{
        common::store_type::StoreType,
        store::{object_store::ObjectStore, standalone_store::StandaloneStore},
    };

    #[fixture]
    fn standalone_store() -> StandaloneStore {
        let test_path = std::env::current_dir()
            .unwrap()
            .join("test_store")
            .join("standalone");
        StandaloneStore::new(test_path)
    }

    #[rstest]
    #[tokio::test]
    async fn test_save_and_open(mut standalone_store: StandaloneStore) {
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
            .await
            .unwrap();

        if let StoreType::Standalone { file_path } = store_type {
            assert!(file_path.file_name().is_some());
            let mut stream = StandaloneStore::open(&file_path).await.unwrap();

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
