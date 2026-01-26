use std::path::PathBuf;

use anyhow::Ok;

use crate::{common::store_type::StoreType, store::object_store::ObjectStore};

pub struct StandaloneStore {}
impl StandaloneStore {
    pub fn new() -> Self {
        Self {}
    }
}

impl ObjectStore for StandaloneStore {
    fn save(&self) -> anyhow::Result<crate::common::store_type::StoreType> {
        println!("saving object to standalone file");
        Ok(StoreType::Standalone {
            file_path: PathBuf::from("./data/standalone/1.store"),
        })
    }
}
