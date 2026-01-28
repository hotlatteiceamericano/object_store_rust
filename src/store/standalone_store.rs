use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use anyhow::Ok;
use axum::body::Bytes;

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

    fn save(&self, bytes: Bytes) -> anyhow::Result<StoreType> {
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
}
