use std::{path::PathBuf, pin::Pin};

use axum::body::Bytes;
use futures::{Stream, io};

use crate::common::store_type::StoreType;

pub trait ObjectStore {
    const STORE_EXTENSION: &'static str = "store";

    fn path() -> PathBuf;
    async fn save(&mut self, bytes: &Bytes) -> anyhow::Result<StoreType>;
}
