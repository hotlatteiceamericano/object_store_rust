use std::{path::PathBuf, pin::Pin};

use axum::body::Bytes;
use futures::{Stream, io};

use crate::common::store_type::StoreType;

pub trait ObjectStore {
    const STORE_EXTENSION: &'static str = "store";

    fn path() -> PathBuf;
    async fn save(&mut self, bytes: &Bytes) -> anyhow::Result<StoreType>;

    /// Purposefully return BoxStream instead of axum::IntoResponse
    /// to decouple the store layer from http layer
    async fn open(
        file_name: &str,
    ) -> anyhow::Result<Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send>>>;
}
