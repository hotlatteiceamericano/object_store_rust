use std::path::PathBuf;

use anyhow::Ok;
use axum::body::Bytes;

use crate::{common::store_type::StoreType, store::object_store::ObjectStore};

pub struct StandaloneStore {}
impl StandaloneStore {
    pub fn new() -> Self {
        Self {}
    }
}

impl ObjectStore for StandaloneStore {
    fn save(&self, bytes: Bytes) -> anyhow::Result<crate::common::store_type::StoreType> {
        println!("saving object to standalone file");
        // find the next incremental file name
        // save the entire bytes to ./data/standalone/ with next incremental file name
        // create metadata according to the filepath
        // store the metadata in a key-value store using "sled"
        // return results
        Ok(StoreType::Standalone {
            file_path: PathBuf::from("./data/standalone/1.store"),
        })
    }
}
