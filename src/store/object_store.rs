use std::path::{self, Path, PathBuf};

use axum::body::Bytes;

use crate::common::store_type::StoreType;

pub trait ObjectStore {
    const STORE_EXTENSION: &'static str = "store";

    fn save(&self, bytes: Bytes) -> anyhow::Result<StoreType>;
    fn path() -> PathBuf;
}
