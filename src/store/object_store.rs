use crate::common::store_type::StoreType;
use axum::body::Bytes;

pub trait ObjectStore {
    const STORE_EXTENSION: &'static str = "store";

    async fn save(&mut self, bytes: &Bytes) -> anyhow::Result<StoreType>;
}
