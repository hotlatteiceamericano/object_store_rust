use std::{path::PathBuf, pin::Pin};

use axum::body::Bytes;
use futures::{Stream, io, stream::BoxStream};

use crate::common::store_type::StoreType;

pub trait ObjectStore {
    const STORE_EXTENSION: &'static str = "store";

    fn path() -> PathBuf;
    async fn save(&self, bytes: &Bytes) -> anyhow::Result<StoreType>;

    // I think the outer Result wrap is confusing but still needed
    // outer Result meant for other errors like file opening
    // inner error meant for the error to read each chunks
    // todo: see how this can be improved
    /// Purposefully return BoxStream instead of axum::IntoResponse
    /// to decouple the store layer from http layer
    async fn open(
        file_name: &str,
        // ) -> anyhow::Result<BoxStream<'static, Result<Bytes, io::Error>>>;
    ) -> anyhow::Result<Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send>>>;
}
