use axum::body::Bytes;

use crate::common::store_type::StoreType;

pub trait ObjectStore {
    fn save(&self, bytes: Bytes) -> anyhow::Result<StoreType>;
}
