use std::path::{self, Path, PathBuf};

use axum::body::Bytes;
use futures::{io, stream::BoxStream};

use crate::common::store_type::StoreType;

pub trait ObjectStore {
    const STORE_EXTENSION: &'static str = "store";

    fn path() -> PathBuf;
    fn save(&self, bytes: &Bytes) -> anyhow::Result<StoreType>;
    async fn open(
        &self,
        file_name: &str,
    ) -> anyhow::Result<BoxStream<'static, Result<Bytes, io::Error>>>;
}
