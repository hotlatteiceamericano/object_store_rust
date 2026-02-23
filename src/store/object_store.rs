use std::path::PathBuf;

use crate::common::store_type::StoreType;
use axum::body::Bytes;

pub trait ObjectStore {
    const STORE_EXTENSION: &'static str = "store";

    fn path() -> PathBuf;
    async fn save(&mut self, bytes: &Bytes) -> anyhow::Result<StoreType>;
}
